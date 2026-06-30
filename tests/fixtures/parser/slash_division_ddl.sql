-- Issue: slash-division-operator
-- Description: / is treated as terminator in DDL statements (CREATE VIEW, CREATE TABLE AS, etc.)
-- Expect: parse
-- Split: semicolon

CREATE VIEW v AS SELECT a / 1000 FROM tab;

CREATE MATERIALIZED VIEW mv AS SELECT a / 1000 FROM tab;

CREATE TABLE t2 AS SELECT a / 1000 FROM tab;
