#!/usr/bin/env bash
# ============================================================
# mine.sh —— 挖掘 ogsql-parser 的新失败到 regress/
# ============================================================
# 用法 / Usage:
#   ./bin/mine.sh [--max-statements N]
#
# 默认对 fixtures/ 下所有 corpus-*.sql 跑 harness mine。
# 新发现的失败（不在已知 regress/ 案例中）会自动入库。
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
FIXTURES_DIR="$ROOT_DIR/fixtures"
REGRESS_DIR="$ROOT_DIR/regress"
KNOWN_FILE="$ROOT_DIR/config/known-acceptable-failures.txt"

EXTRA_ARGS=""
for arg in "$@"; do
    case "$arg" in
        --max-statements)
            shift
            EXTRA_ARGS="$EXTRA_ARGS --max-statements $1"
            ;;
        *)
            EXTRA_ARGS="$EXTRA_ARGS $arg"
            ;;
    esac
done

if [[ ! -d "$FIXTURES_DIR" ]] || [[ -z "$(ls -A "$FIXTURES_DIR" 2>/dev/null)" ]]; then
    cat >&2 <<EOF
ERROR: fixtures/ is empty. Generate corpus first:

  docker compose up -d
  ./bin/corpus-gen.sh
EOF
    exit 1
fi

CORPUS_FILES=( $(ls "$FIXTURES_DIR"/corpus-*.sql 2>/dev/null || true) )
if [[ ${#CORPUS_FILES[@]} -eq 0 ]]; then
    echo "ERROR: no corpus-*.sql under $FIXTURES_DIR" >&2
    exit 1
fi

mkdir -p "$REGRESS_DIR" "$ROOT_DIR/reports"

echo "Mining ${#CORPUS_FILES[@]} corpus file(s) ..."
echo "  fixtures:  $FIXTURES_DIR"
echo "  regress:   $REGRESS_DIR"
echo ""

# shellcheck disable=SC2086
cargo run --bin sqlsmith-harness --features cli -- \
    mine "$FIXTURES_DIR" \
    --out "$REGRESS_DIR" \
    --known "$KNOWN_FILE" \
    --report "$ROOT_DIR/reports" \
    $EXTRA_ARGS

echo ""
echo "Done. New failures (if any) are under:"
echo "  $REGRESS_DIR/"
echo "Reports:"
echo "  $ROOT_DIR/reports/new-failures.csv"
