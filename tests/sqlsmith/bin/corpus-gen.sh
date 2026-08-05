#!/usr/bin/env bash
# ============================================================
# corpus-gen.sh —— 生成 SQLsmith 语料到 fixtures/
# ============================================================
# 用法 / Usage:
#   ./bin/corpus-gen.sh [--force]
#
# 前提 / Prereqs:
#   - docker compose 已经启动 postgres 服务
#     (docker compose up -d postgres)
#   - sqlsmith 通过 docker compose run sqlsmith 调用，
#     宿主机不需要装 sqlsmith
#
# 输出 / Output:
#   fixtures/corpus-s<SEED>-<MAX_QUERIES>.sql
#   fixtures/corpus-s<SEED>-<MAX_QUERIES>.meta.json
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CONFIG_FILE="$ROOT_DIR/config/sqlsmith.conf"

# shellcheck source=/dev/null
source "$CONFIG_FILE"

FORCE=0
for arg in "$@"; do
    case "$arg" in
        --force) FORCE=1 ;;
        *) echo "Unknown arg: $arg" >&2; exit 2 ;;
    esac
done

FIXTURES_DIR="$ROOT_DIR/fixtures"
mkdir -p "$FIXTURES_DIR"

echo "Checking postgres health via docker compose..."
if ! docker compose -f "$ROOT_DIR/docker-compose.yaml" ps postgres | grep -q "healthy"; then
    cat >&2 <<EOF
ERROR: postgres service not healthy. Start it first:

  cd $ROOT_DIR
  docker compose up -d postgres
  # wait ~15s for init schema to complete
EOF
    exit 1
fi

# sqlsmith 容器内通过 compose service DNS 'postgres' 访问数据库（内部端口 5432）
PG_CONNSTR_INTERNAL="host=postgres port=5432 user=${PG_USER} dbname=${PG_DB} password=${PG_PASSWORD}"

EXCLUDE_OPT=""
if [[ "$EXCLUDE_CATALOG" == "1" ]]; then
    EXCLUDE_OPT="--exclude-catalog"
fi

TOTAL_SEEDS=${#SEEDS[@]}
SEED_IDX=0

for seed in "${SEEDS[@]}"; do
    SEED_IDX=$((SEED_IDX + 1))
    OUT_SQL="$FIXTURES_DIR/corpus-s${seed}-${MAX_QUERIES}.sql"
    OUT_META="$FIXTURES_DIR/corpus-s${seed}-${MAX_QUERIES}.meta.json"

    if [[ -f "$OUT_SQL" && "$FORCE" -eq 0 ]]; then
        echo "[$SEED_IDX/$TOTAL_SEEDS] SKIP seed=$seed (exists): $OUT_SQL"
        continue
    fi

    echo "[$SEED_IDX/$TOTAL_SEEDS] Generating seed=$seed max_queries=$MAX_QUERIES ..."

    # --rm 跑完删容器；-T 禁用 pseudo-tty；--no-deps 跳过 depends_on（postgres 已启动）
    docker compose -f "$ROOT_DIR/docker-compose.yaml" run \
        --rm \
        -T \
        --no-deps \
        sqlsmith \
        --dry-run \
        --target="$PG_CONNSTR_INTERNAL" \
        --seed="$seed" \
        --max-queries="$MAX_QUERIES" \
        $EXCLUDE_OPT \
        > "$OUT_SQL"

    LINES=$(wc -l < "$OUT_SQL")
    SQLSMITH_VERSION=$(docker compose -f "$ROOT_DIR/docker-compose.yaml" run \
        --rm -T --no-deps sqlsmith --version 2>&1 | head -1 | tr -d '\n' || echo "unknown")

    cat > "$OUT_META" <<EOF
{
  "seed": $seed,
  "max_queries": $MAX_QUERIES,
  "generated_at": "$(date -u +"%Y-%m-%dT%H:%M:%SZ")",
  "sqlsmith_version": "$(printf '%s' "$SQLSMITH_VERSION" | sed 's/"/\\"/g')",
  "postgres_target": "postgres:16",
  "lines": $LINES,
  "exclude_catalog": $EXCLUDE_CATALOG
}
EOF

    echo "    → $LINES lines, meta: $OUT_META"
done

cat >&2 <<EOF

Done. Generated $TOTAL_SEEDS corpus file(s) under:
  $FIXTURES_DIR/

Next:
  cargo run --bin sqlsmith-harness --features cli -- \\
      mine $FIXTURES_DIR --out $ROOT_DIR/regress/ \\
      --known $ROOT_DIR/config/known-acceptable-failures.txt
EOF
