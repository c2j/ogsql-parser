//! Integration tests for the `serve-stdio` NDJSON line protocol.
//!
//! Spawns the real `ogsql serve-stdio` binary and drives it over pipes,
//! mirroring how a Java client would embed the parser.
#![cfg(feature = "cli")]

use serde_json::{json, Value};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const TIMEOUT: Duration = Duration::from_secs(15);

/// A spawned `serve-stdio` server with a background stdout reader thread.
struct Server {
    child: Child,
    stdin: BufWriter<ChildStdin>,
    responses: mpsc::Receiver<Result<String, std::io::Error>>,
}

impl Server {
    fn start() -> Server {
        let bin = env!("CARGO_BIN_EXE_ogsql");
        let mut child = Command::new(bin)
            .arg("serve-stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("failed to spawn ogsql serve-stdio");
        let stdin = child.stdin.take().expect("stdin");
        let stdout = child.stdout.take().expect("stdout");

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if tx.send(line).is_err() {
                    break;
                }
            }
            let _ = tx.send(Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "serve-stdio stdout closed")));
        });

        Server { child, stdin: BufWriter::new(stdin), responses: rx }
    }

    /// Send one request line and block for its response line.
    fn call(&mut self, req: Value) -> Value {
        let line = serde_json::to_string(&req).unwrap();
        self.stdin.write_all(line.as_bytes()).unwrap();
        self.stdin.write_all(b"\n").unwrap();
        self.stdin.flush().unwrap();
        let resp = self
            .responses
            .recv_timeout(TIMEOUT)
            .expect("timed out waiting for serve-stdio response")
            .expect("failed reading serve-stdio response");
        serde_json::from_str(&resp).expect("serve-stdio returned invalid JSON")
    }

    /// Write a raw (possibly non-JSON) line, then read the next response line.
    fn raw_call(&mut self, raw: &str) -> Value {
        self.stdin.write_all(raw.as_bytes()).unwrap();
        self.stdin.write_all(b"\n").unwrap();
        self.stdin.flush().unwrap();
        let resp = self
            .responses
            .recv_timeout(TIMEOUT)
            .expect("timed out waiting for serve-stdio response")
            .expect("failed reading serve-stdio response");
        serde_json::from_str(&resp).expect("serve-stdio returned invalid JSON")
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

// ─── hello / ping / shutdown ─────────────────────────────────

#[test]
fn hello_returns_version_and_ops() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 1, "op": "hello" }));
    assert_eq!(r["ok"], true);
    assert_eq!(r["id"], 1);
    assert_eq!(r["result"]["protocol"], 1);
    assert!(r["result"]["version"].as_str().unwrap().len() > 0);
    let ops = r["result"]["ops"].as_array().unwrap();
    for op in ["hello", "ping", "shutdown", "parse", "format", "tokenize", "validate", "json2sql"] {
        assert!(ops.iter().any(|o| o == op), "missing op {op}");
    }
}

#[test]
fn ping_pong() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 15, "op": "ping" }));
    assert_eq!(r["result"]["pong"], true);
}

#[test]
fn shutdown_exits_cleanly() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 16, "op": "shutdown" }));
    assert_eq!(r["ok"], true);
    let status = s.child.wait().expect("wait for child");
    assert!(status.success(), "serve-stdio should exit 0 after shutdown");
}

// ─── parse ───────────────────────────────────────────────────

#[test]
fn parse_valid_sql() {
    let mut s = Server::start();
    let r = s.call(json!({
        "id": 2,
        "op": "parse",
        "sql": "SELECT id, name FROM users WHERE status = 'active'",
    }));
    assert_eq!(r["ok"], true);
    assert_eq!(r["result"]["statements"].as_array().unwrap().len(), 1);
    assert_eq!(r["result"]["errors"].as_array().unwrap().len(), 0);
    assert!(r["result"]["query_fingerprints"].is_array());
}

#[test]
fn parse_invalid_sql_reports_errors_not_protocol_error() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 3, "op": "parse", "sql": "SELECT FROM WHERE" }));
    assert_eq!(r["ok"], true, "syntax problems are not protocol errors");
    assert!(r["result"]["errors"].as_array().unwrap().len() > 0);
}

#[test]
fn parse_mybatis_placeholder() {
    let mut s = Server::start();
    let r = s.call(json!({
        "id": 4,
        "op": "parse",
        "sql": "SELECT * FROM t WHERE id = #{userId}",
        "mybatis": true,
    }));
    assert_eq!(r["ok"], true);
    assert_eq!(r["result"]["errors"].as_array().unwrap().len(), 0);
}

#[test]
fn parse_procedure_filter() {
    let mut s = Server::start();
    let r = s.call(json!({
        "id": 20,
        "op": "parse",
        "sql": "SELECT 1",
        "procedure": "does_not_exist",
    }));
    assert_eq!(r["ok"], false);
    assert_eq!(r["error"]["code"], "NOT_FOUND");
}

// ─── format / tokenize / validate / json2sql ─────────────────

#[test]
fn format_upper_keyword_case() {
    let mut s = Server::start();
    let r = s.call(json!({
        "id": 5,
        "op": "format",
        "sql": "select 1",
        "keyword_case": "upper",
    }));
    assert_eq!(r["ok"], true);
    let sql = r["result"]["sql"].as_str().unwrap();
    assert!(sql.contains("SELECT 1"), "got: {sql}");
}

#[test]
fn format_mybatis_preserved() {
    let mut s = Server::start();
    let r = s.call(json!({
        "id": 21,
        "op": "format",
        "sql": "select * from t where id = #{x}",
        "mybatis": true,
    }));
    assert_eq!(r["ok"], true);
    assert!(r["result"]["sql"].as_str().unwrap().contains("#{x}"));
}

#[test]
fn tokenize_sql() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 6, "op": "tokenize", "sql": "SELECT 1" }));
    assert_eq!(r["ok"], true);
    let tokens = r["result"]["tokens"].as_array().unwrap();
    assert!(!tokens.is_empty());
    assert!(tokens[0].get("type").is_some() && tokens[0].get("value").is_some());
}

#[test]
fn validate_invalid_sql() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 7, "op": "validate", "sql": "SELECT FROM WHERE" }));
    assert_eq!(r["ok"], true);
    assert_eq!(r["result"]["valid"], false);
    assert!(r["result"]["errors"].as_array().unwrap().len() > 0);
}

#[test]
fn validate_valid_sql() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 8, "op": "validate", "sql": "SELECT 1" }));
    assert_eq!(r["ok"], true);
    assert_eq!(r["result"]["valid"], true);
}

#[test]
fn json2sql_roundtrip() {
    let mut s = Server::start();
    let p = s.call(json!({ "id": 9, "op": "parse", "sql": "SELECT a, b FROM t WHERE c = 1" }));
    assert_eq!(p["ok"], true);
    let ast = serde_json::to_string(&p["result"]).unwrap();
    let r = s.call(json!({ "id": 10, "op": "json2sql", "json": ast }));
    assert_eq!(r["ok"], true);
    let sql = r["result"]["sql"].as_str().unwrap();
    assert!(sql.to_uppercase().contains("SELECT"), "got: {sql}");
}

// ─── protocol-level failures ─────────────────────────────────

#[test]
fn unknown_op_fails() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 11, "op": "nope" }));
    assert_eq!(r["ok"], false);
    assert_eq!(r["error"]["code"], "UNKNOWN_OP");
}

#[test]
fn invalid_json_line_fails_without_killing_server() {
    let mut s = Server::start();
    let r = s.raw_call("{not json}");
    assert_eq!(r["ok"], false);
    assert_eq!(r["error"]["code"], "PROTOCOL_ERROR");
    // The server must keep serving afterwards.
    let r2 = s.call(json!({ "id": 17, "op": "ping" }));
    assert_eq!(r2["result"]["pong"], true);
}

#[test]
fn missing_required_param_fails() {
    let mut s = Server::start();
    let r = s.call(json!({ "id": 12, "op": "parse" }));
    assert_eq!(r["ok"], false);
    assert_eq!(r["error"]["code"], "BAD_PARAM");
}

#[test]
fn hostile_input_keeps_server_alive() {
    let mut s = Server::start();
    // Bounded-but-hostile input (deep but under the nesting guard): must parse
    // or report errors, and the server loop must survive.
    let deep = format!("SELECT {}", "(".repeat(30)) + "1" + &")".repeat(30);
    let r = s.call(json!({ "id": 13, "op": "parse", "sql": deep }));
    assert_eq!(r["ok"], true);
    let r2 = s.call(json!({ "id": 14, "op": "ping" }));
    assert_eq!(r2["result"]["pong"], true);
}

#[test]
fn excessive_nesting_rejected_gracefully() {
    let mut s = Server::start();
    // Unbounded nesting would stack-overflow the recursive-descent parser and
    // abort the process (uncatchable). The protocol server rejects it up front.
    let deep = format!("SELECT {}", "(".repeat(500)) + "1" + &")".repeat(500);
    let r = s.call(json!({ "id": 22, "op": "parse", "sql": deep }));
    assert_eq!(r["ok"], false);
    assert_eq!(r["error"]["code"], "TOO_DEEP");
    // The server must still be alive and serving.
    let r2 = s.call(json!({ "id": 23, "op": "ping" }));
    assert_eq!(r2["result"]["pong"], true);
}
