package io.github.c2j.ogsql;

import com.fasterxml.jackson.databind.JsonNode;
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.databind.node.ObjectNode;

import java.io.BufferedReader;
import java.io.BufferedWriter;
import java.io.IOException;
import java.io.InputStreamReader;
import java.io.OutputStreamWriter;
import java.nio.charset.StandardCharsets;
import java.nio.file.Path;
import java.util.concurrent.BlockingQueue;
import java.util.concurrent.LinkedBlockingQueue;
import java.util.concurrent.TimeUnit;
import java.util.concurrent.atomic.AtomicLong;
import java.util.logging.Level;
import java.util.logging.Logger;

/**
 * Owns the {@code ogsql serve-stdio} child process and speaks the NDJSON line
 * protocol over its stdin/stdout (see {@code docs/stdio-protocol.md}).
 *
 * <p><strong>Serial model:</strong> exactly one request is in flight at a time —
 * the write lock is held across the whole round trip, and responses are read
 * strictly in order from a FIFO queue (the server is serial, so the head of the
 * queue is always the response to the request just written). Concurrency-safe:
 * concurrent callers serialize on the lock.
 *
 * <p><strong>Resilience:</strong> a dedicated daemon thread drains stdout (never
 * lets the pipe buffer fill). If the child dies (stdout EOF), in-flight and
 * subsequent calls fail with {@link OgsqlException}; with restart enabled the
 * process is respawned with exponential backoff and the failed call is retried
 * once — parse/format/validate/tokenize/json2sql are pure, so retry is safe.
 */
final class OgsqlProcessManager implements AutoCloseable {

    static final int PROTOCOL_VERSION = 1;

    private static final ObjectMapper MAPPER = new ObjectMapper();
    private static final Object POISON = new Object();
    private static final Logger LOG = Logger.getLogger("io.github.c2j.ogsql");

    private final long timeoutMillis;
    private final int maxRestarts;
    private final AtomicLong idSeq = new AtomicLong(1);

    private final Object lock = new Object(); // serializes the whole round trip
    private final BlockingQueue<Object> responses = new LinkedBlockingQueue<>();

    private Process process;
    private BufferedWriter stdin;
    private Thread readerThread;
    private volatile boolean closed;
    private volatile boolean processDead;
    private volatile String version = "unknown";
    private volatile int protocol;

    OgsqlProcessManager(Path binary, long timeoutMillis, int maxRestarts) throws IOException {
        this.timeoutMillis = timeoutMillis;
        this.maxRestarts = maxRestarts;
        spawn(binary);
        handshake();
        Runtime.getRuntime().addShutdownHook(new Thread(this::close, "ogsql-shutdown"));
    }

    private void spawn(Path binary) throws IOException {
        ProcessBuilder pb = new ProcessBuilder(binary.toString(), "serve-stdio");
        pb.redirectErrorStream(false);
        Process p = pb.start();
        BufferedWriter w = new BufferedWriter(new OutputStreamWriter(p.getOutputStream(), StandardCharsets.UTF_8));
        BufferedReader r = new BufferedReader(new InputStreamReader(p.getInputStream(), StandardCharsets.UTF_8));
        BufferedReader err = new BufferedReader(new InputStreamReader(p.getErrorStream(), StandardCharsets.UTF_8));

        Thread t = new Thread(() -> pump(r), "ogsql-stdio-reader");
        t.setDaemon(true);
        t.start();
        // The child logs to stderr; forward it to java.util.logging so the
        // protocol channel on stdout stays clean but diagnostics are visible.
        Thread errT = new Thread(() -> pumpStderr(err), "ogsql-stderr-reader");
        errT.setDaemon(true);
        errT.start();

        this.process = p;
        this.stdin = w;
        this.readerThread = t;
        this.processDead = false;
        // Any leftovers from a previous incarnation must not be routed to new calls.
        responses.clear();
    }

    private void pump(BufferedReader reader) {
        try {
            String line;
            while ((line = reader.readLine()) != null) {
                responses.add(line);
            }
        } catch (IOException ignored) {
            // falls through to EOF handling
        }
        processDead = true;
        responses.add(POISON);
    }

    private void pumpStderr(BufferedReader reader) {
        try {
            String line;
            while ((line = reader.readLine()) != null) {
                LOG.fine("[ogsql] " + line);
            }
        } catch (IOException ignored) {
            // child stderr closed
        }
    }

    private void handshake() {
        JsonNode result = callRaw("hello");
        protocol = result.path("protocol").asInt(-1);
        if (protocol != PROTOCOL_VERSION) {
            close();
            throw new OgsqlException("protocol mismatch: serve-stdio speaks protocol "
                + protocol + ", this connector requires " + PROTOCOL_VERSION);
        }
        version = result.path("version").asText("unknown");
    }

    String version() {
        return version;
    }

    int protocol() {
        return protocol;
    }

    boolean isAlive() {
        return !closed && !processDead && process.isAlive();
    }

    /** Package-private: raw child process (tests use this to simulate a crash). */
    Process process() {
        return process;
    }

    /** Send one op request and return its {@code result} object. */
    JsonNode call(String op, ObjectNode params) throws OgsqlException {
        ObjectNode req = MAPPER.createObjectNode();
        req.put("op", op);
        if (params != null) {
            req.setAll(params);
        }
        return callRaw(op, req);
    }

    private JsonNode callRaw(String op) {
        return callRaw(op, null);
    }

    private JsonNode callRaw(String op, ObjectNode req) {
        int attempts = 0;
        while (true) {
            try {
                return roundTrip(op, req);
            } catch (OgsqlException e) {
                if (!e.code().equals("PROCESS_EXITED") || closed || attempts >= maxRestarts) {
                    if (attempts > 0) {
                        throw new OgsqlException("PROCESS_EXITED",
                            "serve-stdio exited; restart limit (" + maxRestarts + ") reached for op '" + op + "'");
                    }
                    throw e;
                }
                attempts++;
                restartWithBackoff(attempts);
                // retry the same pure op (parse/format/validate/tokenize/json2sql are side-effect free)
            }
        }
    }

    private JsonNode roundTrip(String op, ObjectNode req) throws OgsqlException {
        ensureOpen();
        long id = idSeq.getAndIncrement();
        if (req == null) {
            req = MAPPER.createObjectNode();
        }
        req.put("id", id);
        req.put("op", op);

        Object response;
        synchronized (lock) {
            try {
                stdin.write(MAPPER.writeValueAsString(req));
                stdin.newLine();
                stdin.flush();
            } catch (IOException e) {
                processDead = true;
                throw new OgsqlException("PROCESS_EXITED", "failed writing to serve-stdio: " + e.getMessage());
            }
            try {
                response = responses.poll(timeoutMillis, TimeUnit.MILLISECONDS);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                throw new OgsqlException("interrupted while waiting for serve-stdio response", e);
            }
        }

        if (response == POISON) {
            throw new OgsqlException("PROCESS_EXITED", "serve-stdio process exited (stdout EOF)");
        }
        if (response == null) {
            throw new OgsqlException("TIMEOUT", "timed out after " + timeoutMillis + " ms waiting for op '" + op + "'");
        }
        JsonNode resp;
        try {
            resp = MAPPER.readTree((String) response);
        } catch (IOException e) {
            throw new OgsqlException("INVALID_RESPONSE", "serve-stdio returned non-JSON line: " + response);
        }
        if (!resp.path("ok").asBoolean(false)) {
            String code = resp.path("error").path("code").asText("UNKNOWN");
            String message = resp.path("error").path("message").asText("unknown error");
            throw new OgsqlException(code, message);
        }
        return resp.path("result");
    }

    private void ensureOpen() {
        if (closed) {
            throw new OgsqlException("connector is closed");
        }
    }

    private void restartWithBackoff(int attempt) {
        synchronized (lock) {
            if (closed) {
                return;
            }
            try {
                long backoff = Math.min(100L * (1L << (attempt - 1)), 5000L);
                Thread.sleep(backoff);
                spawn(NativeBinaryLoader.resolveBinary());
                handshake();
            } catch (IOException | OgsqlException e) {
                close();
                throw new OgsqlException("failed to restart serve-stdio (attempt " + attempt + "): " + e.getMessage(), e);
            } catch (InterruptedException e) {
                Thread.currentThread().interrupt();
                close();
                throw new OgsqlException("interrupted while restarting serve-stdio", e);
            }
        }
    }

    @Override
    public synchronized void close() {
        if (closed) {
            return;
        }
        closed = true;
        try {
            synchronized (lock) {
                stdin.write("{\"id\":-2,\"op\":\"shutdown\"}\n");
                stdin.flush();
                responses.poll(1, TimeUnit.SECONDS);
            }
        } catch (Exception ignored) {
            // process may already be gone; destroy() below is the fallback
        }
        process.destroy();
        try {
            if (!process.waitFor(2, TimeUnit.SECONDS)) {
                process.destroyForcibly();
            }
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
            process.destroyForcibly();
        }
        readerThread.interrupt();
    }
}
