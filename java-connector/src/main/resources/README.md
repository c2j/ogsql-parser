# Platform binaries

The Java connector ships the `ogsql` binary (built with `--features cli`) inside the jar,
DuckDB-style, using this naming scheme:

| Platform | Resource path |
|---|---|
| Linux x86_64 | `/ogsql_linux_amd64` |
| Linux arm64 | `/ogsql_linux_arm64` |
| macOS x86_64 | `/ogsql_osx_amd64` |
| macOS arm64 | `/ogsql_osx_arm64` |
| Windows amd64 | `/ogsql_windows_amd64.exe` |

The release workflow (CI) drops the freshly built binaries here before `mvn package`.
At runtime the loader picks the resource for the current `os.name`/`os.arch`, unpacks it to a
temp file and spawns `ogsql serve-stdio`. Deployments that manage the binary themselves use
`-Dogsql.lib.path=/path/to/ogsql` instead (the `-nolib` analogue).

Local development: tests fall back to `../target/{debug,release}/ogsql` when the property is unset.
