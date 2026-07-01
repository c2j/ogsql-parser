# 生态工具规则映射 (Ecosystem Rule Mapping)

本文档作为 ogsql-parser、metamorphosis、ogexplain-analyzer 三工具间的中性参考。根据 M-ARCH-01 架构原则（"禁止反向依赖"），ogsql-parser 作为基础层，仅输出纯规则违规（rule_id、level、suggestion），不嵌入任何上层工具的引用。上层工具（metamorphosis、ogexplain-analyzer）消费 ogsql-parser 输出，并可根据自身能力添加标注。

## 架构关系

```
ogsql-parser (base — lint rules R*/P*/C*/S*)
      ▲
      │ consumed by
      ├── metamorphosis (rewrite engine — auto-fix)
      ├── ogexplain-analyzer (runtime plan diagnosis)
      └── verieql (bounded equivalence verification)
```

## 规则映射表

| ogsql Rule ID | ogsql Rule Name | ogsql Severity | metamorphosis Rewrite Capability | ogexplain-analyzer Runtime Check |
|--------------|----------------|----------------|----------------------------------|--------------------------------|
| R001 | select-star | Prohibition | Safe-level: auto-expand SELECT * to explicit column list | — |
| R005 | implicit-type-conversion | Prohibition (schema-aware) | — | TYPE-001: confirms implicit conversion at runtime |
| R007 | like-leading-wildcard | Prohibition | — | TYPE-004: confirms full scan from leading wildcard |
| R009 | scalar-subquery-in-select | Performance | Manual-level: suggests JOIN rewrite | — |
| R010 | function-side-effect | Prohibition | — | — |
| P001 | union-without-all | Performance | Auto-rewrite: UNION → UNION ALL | — |
| P002 | not-in-subquery | Performance | subquery-to-join: rewrites NOT IN to LEFT JOIN ... IS NULL | — |
| P009 | function-instead-of-case | Performance | nvl-to-case: rewrites NVL/NVL2/DECODE to CASE | — |
| P011 | correlated-subquery | Performance | subquery-to-join: rewrites correlated subquery to JOIN | — |

## 集成模式

ogsql-parser 通过 CLI、HTTP API 或 MCP 接口输出纯规则违规信息：

```json
{
  "rule_id": "R001",
  "level": "prohibition",
  "message": "SELECT * violates coding standards",
  "suggestion": "Expand to explicit column list"
}
```

上层工具读取此输出后，可基于自身能力添加标注：

- **metamorphosis**：识别可自动重写的规则，提供 rewrite 建议（如 P001、P002、P009、P011）
- **ogexplain-analyzer**：识别可在执行计划中确认的规则，提供 runtime 验证（如 R005、R007）
- **verieql**：识别可通过有界等价验证的规则

每个工具在其内部维护独立的映射表；本文档为人工维护的跨工具参考，确保各工具间的映射一致性。

## 维护说明

本文档由 ogsql-parser 维护，作为三工具间的中性参考。当任一工具的规则 ID 或能力变更时，应同步更新此表。
