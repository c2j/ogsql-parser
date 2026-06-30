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

**每次提交前必须执行以下命令，全部通过才能 push：**

```bash
# 1. 格式化（必须运行 cargo fmt，而非仅 check）
cargo fmt --all
cargo fmt --all -- --check   # 确认无差异

# 2. Clippy 检查
cargo clippy --all-features -- -D warnings

# 3. 测试
cargo test --all-features
```

> **注意**：`cargo fmt --all -- --check` 失败是最常见的 CI 错误。
> 新代码、长字符串、assert_eq! 宏参数等都可能需要格式化调整。
> 务必在提交前运行 `cargo fmt --all` 自动修复，然后再 `git add` 变更。

版本号在 `Cargo.toml` 中维护，发布时同步更新。
