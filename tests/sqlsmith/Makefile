# ============================================================
# ogsql-parser / SQLsmith 测试入口
# ============================================================
# 典型用法 / Typical usage:
#
#   make corpus      # 一次性：起 docker + 生成语料
#   make mine        # 挖掘新失败 → regress/
#   make guard       # 守护：对比 expected vs actual，CI 友好
#   make report      # guard + diff baseline
#   make clean       # 清 reports/
#
# 完整初始化 / Full bootstrap:
#   docker compose up -d
#   make corpus
#   make mine
#   docker compose down
# ============================================================

ROOT_DIR := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
HARNESS := cargo run --bin sqlsmith-harness --features cli --
FIXTURES_DIR := $(ROOT_DIR)fixtures
REGRESS_DIR := $(ROOT_DIR)regress
REPORTS_DIR := $(ROOT_DIR)reports
BASELINE_DIR := $(ROOT_DIR)baseline
KNOWN_FILE := $(ROOT_DIR)config/known-acceptable-failures.txt

.PHONY: all corpus mine guard report diff clean help

help:
	@echo "ogsql-parser / SQLsmith test harness"
	@echo ""
	@echo "Targets:"
	@echo "  corpus   Generate SQLsmith fixtures/ via docker postgres"
	@echo "  mine     Mine new failures into regress/"
	@echo "  guard    Run guard (compare expected vs actual)"
	@echo "  report   Run guard + diff baseline"
	@echo "  diff     Alias for report"
	@echo "  clean    Remove reports/"
	@echo ""
	@echo "Prereqs:"
	@echo "  docker compose up -d   (before corpus/mine)"

corpus:
	@cd "$(ROOT_DIR)" && ./bin/corpus-gen.sh

mine:
	@cd "$(ROOT_DIR)" && ./bin/mine.sh

guard:
	@mkdir -p "$(REPORTS_DIR)"
	$(HARNESS) guard \
	    --cases "$(REGRESS_DIR)" \
	    --baseline "$(BASELINE_DIR)/metrics.json" \
	    --report "$(REPORTS_DIR)"

report:
	@cd "$(ROOT_DIR)" && ./bin/report.sh

diff: report

clean:
	@rm -rf "$(REPORTS_DIR)"
	@echo "Cleaned $(REPORTS_DIR)"

all: report
