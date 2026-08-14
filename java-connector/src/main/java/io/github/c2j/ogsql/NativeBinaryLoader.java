package io.github.c2j.ogsql;

import java.io.IOException;
import java.io.InputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.nio.file.StandardCopyOption;
import java.util.Locale;

/**
 * DuckDB-style platform-aware native binary loader.
 *
 * <p>Resolution order (mirrors {@code DuckDBNative}):
 * <ol>
 *   <li>{@code -Dogsql.lib.path=/path/to/ogsql} — explicit external binary (the
 *       {@code -nolib} / {@code java.library.path} analogue; supports hot-swapping
 *       the parser without re-releasing the jar);</li>
 *   <li>bundled jar resource {@code /ogsql_<os>_<arch>} (Windows: {@code .exe}) —
 *       unpacked to a temp file (deleteOnExit) and made executable;</li>
 *   <li>otherwise a descriptive {@link OgsqlException}.</li>
 * </ol>
 * Platform mapping: linux/amd64, linux/arm64, osx/amd64, osx/arm64, windows/amd64.
 */
final class NativeBinaryLoader {

    private NativeBinaryLoader() {
    }

    /** System property override: {@code -Dogsql.lib.path=/path/to/ogsql}. */
    static final String PROP_LIB_PATH = "ogsql.lib.path";

    static String osName() {
        String os = System.getProperty("os.name", "").toLowerCase(Locale.ROOT).trim();
        if (os.startsWith("windows")) {
            return "windows";
        }
        if (os.startsWith("mac")) {
            return "osx";
        }
        if (os.startsWith("linux")) {
            return "linux";
        }
        return os.replaceAll("[^a-z0-9_\\-.]", "");
    }

    static String archName() {
        String arch = System.getProperty("os.arch", "").toLowerCase(Locale.ROOT).trim();
        switch (arch) {
            case "x86_64":
            case "amd64":
                return "amd64";
            case "aarch64":
            case "arm64":
                return "arm64";
            default:
                return arch.replaceAll("[^a-z0-9_\\-.]", "");
        }
    }

    /** Resource name inside the jar, e.g. {@code ogsql_linux_amd64}, {@code ogsql_windows_amd64.exe}. */
    static String resourceName() {
        String base = "ogsql_" + osName() + "_" + archName();
        return osName().equals("windows") ? base + ".exe" : base;
    }

    /**
     * Resolve the ogsql binary: system-property override → bundled resource → error.
     *
     * @return path to an executable ogsql binary
     * @throws OgsqlException when no usable binary can be located
     */
    static Path resolveBinary() throws OgsqlException {
        String override = System.getProperty(PROP_LIB_PATH);
        if (override != null && !override.isBlank()) {
            Path p = Paths.get(override).toAbsolutePath();
            if (!Files.exists(p)) {
                throw new OgsqlException(PROP_LIB_PATH + " points to a missing file: " + p);
            }
            if (!Files.isExecutable(p) && !osName().equals("windows")) {
                throw new OgsqlException(PROP_LIB_PATH + " file is not executable: " + p);
            }
            return p;
        }

        String res = resourceName();
        InputStream in = NativeBinaryLoader.class.getResourceAsStream("/" + res);
        if (in == null) {
            throw new OgsqlException(
                "no bundled ogsql binary for platform '" + res + "' and no -D" + PROP_LIB_PATH + " override. "
                    + "Download/place the platform binary into the jar under /" + res + " or point " + PROP_LIB_PATH
                    + " at an existing ogsql executable.");
        }
        try (InputStream is = in) {
            return unpack(is, res);
        } catch (IOException e) {
            throw new OgsqlException("failed to unpack bundled binary " + res + ": " + e.getMessage(), e);
        }
    }

    private static Path unpack(InputStream in, String resourceName) throws IOException {
        Path tmp = Files.createTempFile("ogsql", suffixFor(resourceName));
        Files.copy(in, tmp, StandardCopyOption.REPLACE_EXISTING);
        tmp.toFile().deleteOnExit();
        // Harmless on officially supported platforms; required on some others.
        tmp.toFile().setExecutable(true, true);
        return tmp;
    }

    private static String suffixFor(String resourceName) {
        int dot = resourceName.lastIndexOf('.');
        return dot >= 0 ? resourceName.substring(dot) : "";
    }
}
