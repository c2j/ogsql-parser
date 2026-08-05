-- ============================================================
-- ogsql-parser / SQLsmith 多样化 schema
-- ============================================================
-- 用途：被 docker-compose 挂载到 /docker-entrypoint-initdb.d/，
-- 在 postgres 容器启动时自动执行。SQLsmith 通过自省这个 schema
-- 来产出尽可能多样的查询语法（含复杂表达式、cast、组合类型、
-- 数组、range、窗口、自定义函数/操作符等）。
--
-- 这份 schema 本身也是 ogsql-parser DDL 解析能力的测试资产。
-- ============================================================

-- 扩展 / Extensions
CREATE EXTENSION IF NOT EXISTS btree_gist;
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS tablefunc;

-- ============================================================
-- 1. 自定义类型 / Custom Types
-- ============================================================
CREATE TYPE rainbow AS ENUM ('red', 'orange', 'yellow', 'green', 'blue', 'indigo', 'violet');

CREATE TYPE complex_number AS (re double precision, im double precision);

CREATE TYPE float_range AS RANGE (
    subtype = float8,
    subtype_diff = float8mi
);

CREATE DOMAIN positive_int AS integer CHECK (VALUE > 0);
CREATE DOMAIN nonempty_text AS text CHECK (length(VALUE) > 0);
CREATE DOMAIN email AS text CHECK (VALUE ~* '^[^@]+@[^@]+\.[^@]+$');

-- ============================================================
-- 2. 序列 / Sequences
-- ============================================================
CREATE SEQUENCE seq_user_id START 1 INCREMENT 1 NO CYCLE;
CREATE SEQUENCE seq_order_id START 1000 CACHE 20;

-- ============================================================
-- 3. 主表 / Core tables —— 覆盖各种类型、约束、默认值
-- ============================================================
CREATE TABLE users (
    id           bigint       PRIMARY KEY DEFAULT nextval('seq_user_id'),
    username     varchar(64)  NOT NULL UNIQUE,
    email_addr   email        NOT NULL,
    age          positive_int,
    score        smallint     DEFAULT 0 CHECK (score >= 0 AND score <= 100),
    balance      numeric(12, 2) DEFAULT 0.00,
    bio          text,
    metadata     jsonb,
    tags         text[],
    coordinates  point,
    favorite_color rainbow DEFAULT 'blue',
    created_at   timestamptz NOT NULL DEFAULT now(),
    updated_at   timestamptz DEFAULT now()
);

CREATE TABLE orders (
    id           bigint       PRIMARY KEY DEFAULT nextval('seq_order_id'),
    user_id      bigint       NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    total        numeric(10, 2) NOT NULL CHECK (total >= 0),
    status       varchar(20)  NOT NULL DEFAULT 'pending',
    items        jsonb,
    placed_at    timestamptz  NOT NULL DEFAULT now(),
    EXCLUDE USING gist (user_id WITH =) WHERE (status = 'pending')
);

CREATE TABLE products (
    id           serial       PRIMARY KEY,
    name         nonempty_text NOT NULL,
    price        numeric(10, 2) NOT NULL,
    attributes   jsonb,
    search_vec   tsvector,
    created_at   timestamptz  DEFAULT now()
);

CREATE TABLE events (
    id           bigserial    PRIMARY KEY,
    event_type   varchar(50)  NOT NULL,
    payload      jsonb,
    occurred_at  timestamptz  NOT NULL DEFAULT now(),
    duration     float_range
);

CREATE TABLE composite_demo (
    id           integer      PRIMARY KEY,
    value        complex_number,
    numbers      integer[],
    matrix       integer[][]
);

-- ============================================================
-- 4. 分区表 / Partitioned tables (declarative)
-- ============================================================
CREATE TABLE measurements (
    id           bigserial,
    log_time     timestamptz  NOT NULL,
    sensor_id    integer      NOT NULL,
    reading      numeric(10, 3),
    PRIMARY KEY (id, log_time)
) PARTITION BY RANGE (log_time);

CREATE TABLE measurements_2024_q1 PARTITION OF measurements
    FOR VALUES FROM ('2024-01-01') TO ('2024-04-01');
CREATE TABLE measurements_2024_q2 PARTITION OF measurements
    FOR VALUES FROM ('2024-04-01') TO ('2024-07-01');

CREATE TABLE events_by_type (
    id           bigserial,
    event_type   varchar(50)  NOT NULL,
    payload      jsonb,
    occurred_at  timestamptz  NOT NULL DEFAULT now()
) PARTITION BY LIST (event_type);

CREATE TABLE events_login PARTITION OF events_by_type
    FOR VALUES IN ('login');
CREATE TABLE events_logout PARTITION OF events_by_type
    FOR VALUES IN ('logout');
CREATE TABLE events_other PARTITION OF events_by_type DEFAULT;

-- ============================================================
-- 5. 索引（多种类型）/ Indexes (variety)
-- ============================================================
CREATE INDEX idx_users_email_lower ON users (lower(email_addr));
CREATE INDEX idx_users_meta_gin ON users USING gin (metadata);
CREATE INDEX idx_users_tags_gin ON users USING gin (tags);
CREATE INDEX idx_users_partial ON users (username) WHERE age IS NOT NULL;
CREATE INDEX idx_products_trgm ON products USING gin (name gin_trgm_ops);
CREATE INDEX idx_orders_user_total ON orders (user_id, total DESC NULLS LAST);
CREATE UNIQUE INDEX idx_users_username_lower ON users (lower(username));

-- ============================================================
-- 6. 视图 / Views
-- ============================================================
CREATE VIEW active_users AS
    SELECT id, username, email_addr, score
    FROM users
    WHERE age IS NOT NULL AND score > 0;

CREATE MATERIALIZED VIEW order_summary AS
    SELECT user_id, count(*) AS order_count, sum(total) AS lifetime_value
    FROM orders
    WHERE status IN ('paid', 'shipped', 'delivered')
    GROUP BY user_id
    WITH DATA;

CREATE UNIQUE INDEX idx_order_summary_user ON order_summary (user_id);

-- ============================================================
-- 7. 自定义函数 / Functions (SQL + PL/pgSQL)
-- ============================================================
CREATE FUNCTION add(a integer, b integer) RETURNS integer AS $$
    SELECT a + b
$$ LANGUAGE sql IMMUTABLE;

CREATE FUNCTION fullname(prefix text, name text) RETURNS text AS $$
    SELECT prefix || ' ' || name
$$ LANGUAGE sql IMMUTABLE;

CREATE OR REPLACE FUNCTION compute_status(total numeric) RETURNS text AS $$
BEGIN
    IF total < 100 THEN
        RETURN 'low';
    ELSIF total < 1000 THEN
        RETURN 'medium';
    ELSE
        RETURN 'high';
    END IF;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

CREATE FUNCTION array_median(arr integer[]) RETURNS integer AS $$
    SELECT percentile_cont(0.5) WITHIN GROUP (ORDER BY x)::integer
    FROM unnest(arr) AS t(x)
$$ LANGUAGE sql IMMUTABLE;

-- ============================================================
-- 8. 自定义聚合 / Custom aggregate
-- ============================================================
CREATE AGGREGATE array_sum (integer[]) (
    sfunc = array_cat,
    stype = integer[],
    initcond = '{}'
);

-- ============================================================
-- 9. 自定义操作符 / Custom operator
-- ============================================================
CREATE FUNCTION complex_add(a complex_number, b complex_number)
RETURNS complex_number AS $$
    SELECT ROW(a.re + b.re, a.im + b.im)::complex_number
$$ LANGUAGE sql IMMUTABLE;

CREATE OPERATOR + (
    LEFTARG = complex_number,
    RIGHTARG = complex_number,
    FUNCTION = complex_add,
    COMMUTATOR = +
);

-- ============================================================
-- 10. 自定义 CAST
-- ============================================================
CREATE FUNCTION int_to_complex(n integer) RETURNS complex_number AS $$
    SELECT ROW(n::float8, 0.0)::complex_number
$$ LANGUAGE sql IMMUTABLE;

CREATE CAST (integer AS complex_number)
    WITH FUNCTION int_to_complex(integer);

-- ============================================================
-- 11. COMMENT ON
-- ============================================================
COMMENT ON TABLE users IS '应用用户主表 / application user master';
COMMENT ON COLUMN users.email_addr IS '邮箱 / email';
COMMENT ON FUNCTION compute_status(numeric) IS '根据金额返回档位';

-- ============================================================
-- 12. 行级安全策略 / Row-Level Security
-- ============================================================
ALTER TABLE users ENABLE ROW LEVEL SECURITY;

CREATE POLICY user_self_select ON users
    FOR SELECT
    USING (true);

CREATE POLICY user_update_self ON users
    FOR UPDATE
    USING (true)
    WITH CHECK (true);

-- ============================================================
-- 13. 外部表占位 (commented out, 需要 FDW)
-- ============================================================
-- CREATE EXTENSION postgres_fdw;
-- CREATE SERVER foreign_server FOREIGN DATA WRAPPER postgres_fdw ...;
-- CREATE FOREIGN TABLE foreign_table (...) SERVER foreign_server;

-- 完成
SELECT 'sqlsmith schema initialized' AS status;
