#!/usr/bin/env bash
# ============================================================
# report.sh —— 守护 + 与 baseline 对比
# ============================================================
# 用法 / Usage:
#   ./bin/report.sh
#
# 行为 / Behavior:
#   1. 调用 sqlsmith-harness guard 遍历 regress/
#   2. 与 baseline/metrics.json 对比
#   3. 输出 regressions.csv / improvements.csv
#   4. 退出码：有 regression → 1，否则 0
# ============================================================
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
REGRESS_DIR="$ROOT_DIR/regress"
BASELINE_DIR="$ROOT_DIR/baseline"
REPORTS_DIR="$ROOT_DIR/reports"

if [[ ! -d "$REGRESS_DIR" ]] || [[ -z "$(ls -A "$REGRESS_DIR" 2>/dev/null | grep -v '^README\|^INDEX' || true)" ]]; then
    cat >&2 <<EOF
ERROR: regress/ has no cases. Run ./bin/mine.sh first.

  ./bin/mine.sh
EOF
    exit 1
fi

mkdir -p "$REPORTS_DIR"

# 调用 harness guard
# harness 自己产出 metrics.json + regressions.csv + improvements.csv
cargo run --bin sqlsmith-harness --features cli -- \
    guard \
    --cases "$REGRESS_DIR" \
    --baseline "$BASELINE_DIR/metrics.json" \
    --report "$REPORTS_DIR"

echo ""
echo "=== Summary ==="
if [[ -f "$REPORTS_DIR/metrics.json" ]]; then
    cat "$REPORTS_DIR/metrics.json"
    echo ""
fi

# 报告回归
if [[ -f "$REPORTS_DIR/regressions.csv" ]]; then
    REG_COUNT=$(($(wc -l < "$REPORTS_DIR/regressions.csv") - 1))
    if [[ "$REG_COUNT" -gt 0 ]]; then
        echo ""
        echo "❌ $REG_COUNT regression(s) detected:"
        cat "$REPORTS_DIR/regressions.csv"
        exit 1
    fi
fi

# 报告改进
if [[ -f "$REPORTS_DIR/improvements.csv" ]]; then
    IMP_COUNT=$(($(wc -l < "$REPORTS_DIR/improvements.csv") - 1))
    if [[ "$IMP_COUNT" -gt 0 ]]; then
        echo ""
        echo "✨ $IMP_COUNT improvement(s) detected — update meta.json to lock in:"
        cat "$REPORTS_DIR/improvements.csv"
    fi
fi

echo ""
echo "✓ No regressions."
