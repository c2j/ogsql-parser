# SQLsmith 测试集成 / SQLsmith Test Integration

> 通过 SQLsmith 的 `--dry-run` 输出，对 ogsql-parser 做差分/回归测试。
> 失败案例沉淀为 `regress/` 守护资产，防止功能改进后悄然回归。
>
> Differential and regression testing of ogsql-parser via SQLsmith `--dry-run`
> output. Failures are archived as guard assets under `regress/` to prevent
> regressions after future parser improvements.

---

## 目录结构 / Layout

```
tests/sqlsmith/
├── README.md                          # 本文档
├── docker-compose.yaml                # postgres + 多样化 schema
├── Makefile                           # 一键入口
├── sqlsmith_harness.rs                # harness 二进制（mine / guard / run）
│
├── config/
│   ├── postgres-init.sql              # 驱动 SQLsmith 多样性的 schema
│   ├── sqlsmith.conf                  # SEEDS / MAX_QUERIES 等
│   └── known-acceptable-failures.txt  # PG-only 语法白名单（前缀/正则）
│
├── bin/
│   ├── corpus-gen.sh                  # 生成 fixtures/corpus-*.sql
│   ├── mine.sh                        # 挖掘新失败 → regress/
│   └── report.sh                      # diff baseline → 报告
│
├── fixtures/                          # 本地生成的完整语料（gitignored）
├── regress/                           # ★ 失败守护案例（入库）
│   └── NNNN-<slug>/{case.sql, meta.json, original.sql}
│
├── baseline/                          # 上次好状态指标（入库）
└── reports/                           # 本次运行报告（gitignored）
```

---

## 工作流 / Workflows

### 首次初始化（一次性）

```sh
cd tests/sqlsmith
docker compose up -d
./bin/corpus-gen.sh                          # 生成 fixtures/
cargo run --bin sqlsmith-harness --features cli -- \
    mine fixtures/ --out regress/ --known config/known-acceptable-failures.txt
# 人工审阅首批失败，标记 known_acceptable=true 的填入 config/known-acceptable-failures.txt
docker compose down
git add regress/ baseline/ config/known-acceptable-failures.txt
```

### 大功能合并前自测

```sh
make guard
# - 若 reports/regressions.csv 非空：修代码
# - 若 reports/improvements.csv 非空：把对应 meta.json 的 expected_outcome 改 "OK"
#   并填 fixed_in_commit
```

### 季度扩语料

```sh
docker compose up -d
./bin/corpus-gen.sh --force
./bin/mine.sh
docker compose down
```

---

## 依赖 / Prerequisites

| 组件 | 版本 | 安装 |
|---|---|---|
| Docker + Docker Compose | v2+ | [docs.docker.com](https://docs.docker.com) |
| Rust | 1.70+ | 与项目主仓库一致 |

> SQLsmith 通过 `Dockerfile.sqlsmith` 容器化运行，宿主机**不需要**单独安装 sqlsmith / psql / pg_isready。docker compose 会自动构建并运行。

---

## harness 命令参考 / Harness CLI

```sh
# 挖掘新失败入 regress/
sqlsmith-harness mine <corpus-dir-or-file>
    [--out regress/]
    [--known config/known-acceptable-failures.txt]
    [--max-statements N]            # 调试：限制扫描条数

# 守护：遍历 regress/，对比 expected vs actual
sqlsmith-harness guard
    [--cases regress/]
    [--baseline baseline/metrics.json]
    [--report reports/]
    # 退出码：有 regression → 非 0

# 一次性扫描出 CSV 报告（不动 regress/）
sqlsmith-harness run <corpus>
    [--report reports/]
```

详细判定矩阵和元数据结构见 [`regress/README.md`](regress/README.md)。

---

## 设计动机 / Why

- ogsql-parser 在 openGauss 官方回归集上 100% 通过，**回归集无法发现盲区**
- SQLsmith 生成 schema-aware、类型正确的 PG 方言 SQL——失败基本就是 ogsql 的真实问题
- 只把**失败案例**入库（不入库成功语料），让 SQLsmith 输出沉淀为长期资产
- `regress/` 的 expected_outcome 字段实现"反退化 + 改进感知"双重守护

完整规划：[`.omo/plans/sqlsmith-harness.md`](../../.omo/plans/sqlsmith-harness.md)
