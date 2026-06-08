# Agents

## Windows 7 Build

ogsql-parser 需要支持 Windows 7 运行环境。Rust 1.78+ 移除了 Windows 7 支持（stdlib 无条件调用 `GetSystemTimePreciseAsFileTime`，该 API 仅在 Windows 8+ 存在）。

### 构建方式

使用官方 Tier 3 目标 `x86_64-win7-windows-msvc`，配合 nightly 工具链和 `-Zbuild-std` 从源码编译标准库：

```bash
# 前置条件
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly

# 构建
cargo +nightly build --release --features cli --target x86_64-win7-windows-msvc -Zbuild-std
cargo +nightly build --release --features full --target x86_64-win7-windows-msvc -Zbuild-std
```

产出物在 `target/x86_64-win7-windows-msvc/release/ogsql.exe`。

32 位目标同理：`i686-win7-windows-msvc`。

### 已有配置

`.cargo/config.toml` 已配置两个 Win7 目标的 `+crt-static` flag，确保静态链接 CRT，避免 MSVC 运行时依赖。

## CI 要求

CI 定义在 `.github/workflows/ci.yml`，包含 4 个 job，全部必须通过：

- **Format**: `cargo fmt --all -- --check` 必须通过
- **Clippy**: `cargo clippy --all-features -- -D warnings` 必须通过
- **Test**: `cargo test --all-features` 必须通过（当前 1772+ 测试）
- **Security Audit**: `cargo audit` 必须通过（无已知漏洞依赖）

### 提交前本地验证

```bash
cargo fmt --all -- --check
cargo clippy --all-features -- -D warnings
cargo test --all-features
```

版本号在 `Cargo.toml` 中维护，发布时同步更新。
