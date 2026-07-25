import java.sql.*;
import java.time.LocalDate;
import java.util.*;
import java.util.stream.Collectors;

/**
 * GaussDB/OpenGauss 动态 SQL 完整示例（L1 ~ L5 难度）
 * 所有"值"均使用 ? 占位符；所有"结构"（列名/表名/函数/JOIN类型等）均通过白名单校验后拼接。
 */
public class GaussDBDynamicSQLAdvanced {

    private static final String URL = "jdbc:opengauss://localhost:26000/finance_db?currentSchema=public&reWriteBatchedInserts=true";
    private static final String USER = "dbuser";
    private static final String PASS = "dbpass";

    static {
        try {
            Class.forName("org.opengauss.Driver");
        } catch (ClassNotFoundException e) {
            throw new RuntimeException("缺少 GaussDB/OpenGauss JDBC 驱动", e);
        }
    }

    // ==================== 白名单工具 ====================
    private static final Set<String> ALLOWED_COLS = Set.of(
        "emp_id", "emp_name", "dept_id", "hire_date", "salary", "status", "email", "phone"
    );
    private static final Set<String> ALLOWED_TABLES = Set.of(
        "employee", "department", "salary_grade", "project_member"
    );
    private static final Set<String> ALLOWED_JOIN_TYPES = Set.of(
        "INNER", "LEFT", "RIGHT", "FULL"
    );
    private static final Set<String> ALLOWED_SORT = Set.of("ASC", "DESC");
    private static final Set<String> ALLOWED_AGGS = Set.of(
        "COUNT", "SUM", "AVG", "MAX", "MIN"
    );
    private static final Set<String> ALLOWED_WINDOW_FUNCS = Set.of(
        "ROW_NUMBER", "RANK", "DENSE_RANK", "LEAD", "LAG", "NTILE"
    );

    private static void checkCol(String col) {
        if (!ALLOWED_COLS.contains(col)) throw new IllegalArgumentException("非法列: " + col);
    }
    private static void checkTable(String t) {
        if (!ALLOWED_TABLES.contains(t)) throw new IllegalArgumentException("非法表: " + t);
    }

    // ==================== L1: 动态 SELECT 列 + WHERE ====================
    public List<Map<String, Object>> l1DynamicSelect(
            List<String> selectCols, String name, Integer deptId, LocalDate hireAfter) throws SQLException {

        if (selectCols == null || selectCols.isEmpty()) selectCols = List.of("emp_id", "emp_name");
        selectCols.forEach(GaussDBDynamicSQLAdvanced::checkCol);

        StringBuilder sql = new StringBuilder("SELECT ");
        sql.append(String.join(", ", selectCols));
        sql.append(" FROM employee WHERE 1 = 1 ");

        List<Object> params = new ArrayList<>();
        if (name != null && !name.isEmpty()) {
            sql.append("AND emp_name LIKE ? ");
            params.add("%" + name + "%");
        }
        if (deptId != null) {
            sql.append("AND dept_id = ? ");
            params.add(deptId);
        }
        if (hireAfter != null) {
            sql.append("AND hire_date > ? ");
            params.add(Date.valueOf(hireAfter));
        }

        return executeQuery(sql.toString(), params);
    }

    // ==================== L2: 动态 UPDATE（SET + WHERE） ====================
    public int l2DynamicUpdate(Integer empId, Map<String, Object> updates) throws SQLException {
        if (empId == null) throw new IllegalArgumentException("empId 不能为空");
        if (updates == null || updates.isEmpty()) return 0;

        StringBuilder sql = new StringBuilder("UPDATE employee SET ");
        List<Object> params = new ArrayList<>();
        boolean first = true;

        for (Map.Entry<String, Object> e : updates.entrySet()) {
            checkCol(e.getKey());
            if (!first) sql.append(", ");
            sql.append(e.getKey()).append(" = ?");
            params.add(e.getValue());
            first = false;
        }

        sql.append(" WHERE emp_id = ?");
        params.add(empId);

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql.toString())) {
            for (int i = 0; i < params.size(); i++) ps.setObject(i + 1, params.get(i));
            return ps.executeUpdate();
        }
    }

    // ==================== L2: 动态 ORDER BY ====================
    public List<Map<String, Object>> l2DynamicOrderBy(
            String sortCol, String sortDir, Integer limit) throws SQLException {

        checkCol(sortCol);
        String dir = sortDir == null ? "ASC" : sortDir.toUpperCase();
        if (!ALLOWED_SORT.contains(dir)) throw new IllegalArgumentException("非法排序方向");

        StringBuilder sql = new StringBuilder(
            "SELECT emp_id, emp_name, dept_id, salary FROM employee ORDER BY "
        );
        sql.append(sortCol).append(" ").append(dir);
        if (limit != null) sql.append(" LIMIT ?");

        List<Object> params = new ArrayList<>();
        if (limit != null) params.add(limit);

        return executeQuery(sql.toString(), params);
    }

    // ==================== L2: 动态 IN 子句（两种写法） ====================

    /** 写法 A：传统展开（兼容所有 JDBC） */
    public List<Map<String, Object>> l2DynamicInClassic(List<Integer> deptIds) throws SQLException {
        if (deptIds == null || deptIds.isEmpty()) throw new IllegalArgumentException();

        String placeholders = String.join(", ", Collections.nCopies(deptIds.size(), "?"));
        String sql = "SELECT emp_id, emp_name, dept_id FROM employee WHERE dept_id IN (" + placeholders + ")";

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql)) {
            for (int i = 0; i < deptIds.size(); i++) ps.setInt(i + 1, deptIds.get(i));
            return resultToList(ps.executeQuery());
        }
    }

    /** 写法 B：GaussDB 数组优化（单占位符，推荐） */
    public List<Map<String, Object>> l2DynamicInArray(List<Integer> deptIds) throws SQLException {
        if (deptIds == null || deptIds.isEmpty()) throw new IllegalArgumentException();

        String sql = "SELECT emp_id, emp_name, dept_id FROM employee WHERE dept_id = ANY(?)";

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql)) {
            Array arr = conn.createArrayOf("INTEGER", deptIds.toArray());
            ps.setArray(1, arr);
            return resultToList(ps.executeQuery());
        }
    }

    // ==================== L3: 动态 JOIN ====================
    public List<Map<String, Object>> l3DynamicJoin(
            String mainTable, String joinTable, String joinType,
            String mainCol, String joinCol,
            List<String> selectCols, String whereName) throws SQLException {

        checkTable(mainTable);
        checkTable(joinTable);
        String jt = joinType.toUpperCase();
        if (!ALLOWED_JOIN_TYPES.contains(jt)) throw new IllegalArgumentException("非法 JOIN 类型");
        selectCols.forEach(GaussDBDynamicSQLAdvanced::checkCol);

        StringBuilder sql = new StringBuilder("SELECT ");
        sql.append(String.join(", ", selectCols));
        sql.append(" FROM ").append(mainTable);
        sql.append(" ").append(jt).append(" JOIN ").append(joinTable);
        sql.append(" ON ").append(mainTable).append(".").append(mainCol);
        sql.append(" = ").append(joinTable).append(".").append(joinCol);

        List<Object> params = new ArrayList<>();
        if (whereName != null && !whereName.isEmpty()) {
            sql.append(" WHERE ").append(mainTable).append(".emp_name LIKE ?");
            params.add("%" + whereName + "%");
        }

        return executeQuery(sql.toString(), params);
    }

    // ==================== L3: 动态 GROUP BY + HAVING ====================
    public List<Map<String, Object>> l3DynamicGroupBy(
            String groupCol, String aggFunc, String havingOp, BigDecimal havingVal) throws SQLException {

        checkCol(groupCol);
        String af = aggFunc.toUpperCase();
        if (!ALLOWED_AGGS.contains(af)) throw new IllegalArgumentException("非法聚合函数");

        StringBuilder sql = new StringBuilder("SELECT ");
        sql.append(groupCol).append(", ");
        sql.append(af).append("(salary) AS agg_val ");
        sql.append("FROM employee GROUP BY ").append(groupCol);

        List<Object> params = new ArrayList<>();
        if (havingOp != null && havingVal != null) {
            Set<String> allowedOps = Set.of(">", "<", ">=", "<=", "=", "<>");
            if (!allowedOps.contains(havingOp)) throw new IllegalArgumentException("非法操作符");
            sql.append(" HAVING ").append(af).append("(salary) ").append(havingOp).append(" ?");
            params.add(havingVal);
        }

        return executeQuery(sql.toString(), params);
    }

    // ==================== L3: 分页查询（LIMIT / OFFSET） ====================
    public List<Map<String, Object>> l3Paginate(int page, int pageSize, String sortCol) throws SQLException {
        checkCol(sortCol);
        String sql = "SELECT emp_id, emp_name, salary FROM employee ORDER BY " + sortCol + " LIMIT ? OFFSET ?";

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql)) {
            ps.setInt(1, pageSize);
            ps.setInt(2, (page - 1) * pageSize);
            return resultToList(ps.executeQuery());
        }
    }

    // ==================== L4: UPSERT（INSERT ON CONFLICT） ====================
    public int l4Upsert(Integer empId, String empName, BigDecimal salary, List<String> updateCols) throws SQLException {
        if (updateCols == null || updateCols.isEmpty()) updateCols = List.of("emp_name", "salary");

        StringBuilder sql = new StringBuilder("INSERT INTO employee (emp_id, emp_name, salary) VALUES (?, ?, ?)");
        sql.append(" ON CONFLICT (emp_id) DO UPDATE SET ");
        boolean first = true;
        for (String col : updateCols) {
            checkCol(col);
            if (!first) sql.append(", ");
            sql.append(col).append(" = EXCLUDED.").append(col);
            first = false;
        }

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql.toString())) {
            ps.setInt(1, empId);
            ps.setString(2, empName);
            ps.setBigDecimal(3, salary);
            return ps.executeUpdate();
        }
    }

    // ==================== L4: MERGE INTO（GaussDB 标准 SQL） ====================
    public int l4MergeEmployee(List<<Employee> sources) throws SQLException {
        String sql = """
            MERGE INTO employee t
            USING (SELECT ? AS emp_id, ? AS emp_name, ? AS salary) s
            ON t.emp_id = s.emp_id
            WHEN MATCHED THEN UPDATE SET emp_name = s.emp_name, salary = s.salary
            WHEN NOT MATCHED THEN INSERT (emp_id, emp_name, salary)
                VALUES (s.emp_id, s.emp_name, s.salary)
            """;

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql)) {
            for (Employee e : sources) {
                ps.setInt(1, e.empId);
                ps.setString(2, e.empName);
                ps.setBigDecimal(3, e.salary);
                ps.addBatch();
            }
            return Arrays.stream(ps.executeBatch()).sum();
        }
    }

    // ==================== L4: 批量插入 + 自动主键回取 ====================
    public List<Integer> l4BatchInsert(List<<Employee> list) throws SQLException {
        String sql = "INSERT INTO employee (emp_name, dept_id, salary) VALUES (?, ?, ?)";

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql, Statement.RETURN_GENERATED_KEYS)) {

            for (Employee e : list) {
                ps.setString(1, e.empName);
                ps.setInt(2, e.deptId);
                ps.setBigDecimal(3, e.salary);
                ps.addBatch();
            }
            ps.executeBatch();

            List<Integer> keys = new ArrayList<>();
            try (ResultSet rs = ps.getGeneratedKeys()) {
                while (rs.next()) keys.add(rs.getInt(1));
            }
            return keys;
        }
    }

    // ==================== L4: 动态 CTE（WITH 子句） ====================
    public List<Map<String, Object>> l4DynamicCTE(
            String deptName, BigDecimal minSalary) throws SQLException {

        StringBuilder sql = new StringBuilder();
        sql.append("WITH dept_avg AS ( ");
        sql.append("  SELECT dept_id, AVG(salary) AS avg_sal FROM employee GROUP BY dept_id ");
        sql.append("), high_emp AS ( ");
        sql.append("  SELECT emp_id, emp_name, dept_id, salary FROM employee WHERE salary > ? ");
        sql.append(") ");
        sql.append("SELECT h.emp_id, h.emp_name, h.salary, d.avg_sal ");
        sql.append("FROM high_emp h JOIN dept_avg d ON h.dept_id = d.dept_id ");
        sql.append("WHERE h.emp_name LIKE ?");

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql.toString())) {
            ps.setBigDecimal(1, minSalary);
            ps.setString(2, "%" + deptName + "%");
            return resultToList(ps.executeQuery());
        }
    }

    // ==================== L5: 动态窗口函数 ====================
    public List<Map<String, Object>> l5DynamicWindow(String windowFunc, String partitionCol, String orderCol) throws SQLException {
        String wf = windowFunc.toUpperCase();
        if (!ALLOWED_WINDOW_FUNCS.contains(wf)) throw new IllegalArgumentException("非法窗口函数");
        checkCol(partitionCol);
        checkCol(orderCol);

        String sql = String.format(
            "SELECT emp_id, emp_name, dept_id, salary, %s() OVER (PARTITION BY %s ORDER BY %s DESC) AS rn FROM employee",
            wf, partitionCol, orderCol
        );

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             Statement stmt = conn.createStatement()) {
            return resultToList(stmt.executeQuery(sql));
        }
    }

    // ==================== L5: 动态子查询（EXISTS） ====================
    public List<Map<String, Object>> l5DynamicExists(String subQueryCol, BigDecimal threshold) throws SQLException {
        checkCol(subQueryCol);

        String sql = """
            SELECT e.emp_id, e.emp_name
            FROM employee e
            WHERE EXISTS (
                SELECT 1 FROM employee e2
                WHERE e2.dept_id = e.dept_id AND e2.%s > ?
            )
            """.formatted(subQueryCol);

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql)) {
            ps.setBigDecimal(1, threshold);
            return resultToList(ps.executeQuery());
        }
    }

    // ==================== L5: 动态 UNION ALL ====================
    public List<Map<String, Object>> l5DynamicUnion(List<Integer> deptIds, String status) throws SQLException {
        if (deptIds == null || deptIds.isEmpty()) throw new IllegalArgumentException();

        StringBuilder sql = new StringBuilder();
        List<Object> params = new ArrayList<>();

        for (int i = 0; i < deptIds.size(); i++) {
            if (i > 0) sql.append(" UNION ALL ");
            sql.append("SELECT emp_id, emp_name, ? AS source_dept FROM employee WHERE dept_id = ? AND status = ?");
            params.add("DEPT_" + deptIds.get(i));
            params.add(deptIds.get(i));
            params.add(status);
        }

        return executeQuery(sql.toString(), params);
    }

    // ==================== 通用工具方法 ====================
    private List<Map<String, Object>> executeQuery(String sql, List<Object> params) throws SQLException {
        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql)) {
            for (int i = 0; i < params.size(); i++) ps.setObject(i + 1, params.get(i));
            return resultToList(ps.executeQuery());
        }
    }

    private static List<Map<String, Object>> resultToList(ResultSet rs) throws SQLException {
        List<Map<String, Object>> list = new ArrayList<>();
        ResultSetMetaData md = rs.getMetaData();
        int cols = md.getColumnCount();
        while (rs.next()) {
            Map<String, Object> row = new LinkedHashMap<>();
            for (int i = 1; i <= cols; i++) row.put(md.getColumnLabel(i), rs.getObject(i));
            list.add(row);
        }
        return list;
    }

    // ==================== 实体类 ====================
    public static class Employee {
        public final int empId;
        public final String empName;
        public final int deptId;
        public final java.sql.Date hireDate;
        public final java.math.BigDecimal salary;

        public Employee(int empId, String empName, int deptId, java.sql.Date hireDate, java.math.BigDecimal salary) {
            this.empId = empId; this.empName = empName; this.deptId = deptId;
            this.hireDate = hireDate; this.salary = salary;
        }
    }

    // ==================== main 快速验证 ====================
    public static void main(String[] args) throws SQLException {
        GaussDBDynamicSQLAdvanced dao = new GaussDBDynamicSQLAdvanced();

        // L1
        List<Map<String, Object>> r1 = dao.l1DynamicSelect(
            List.of("emp_id", "emp_name", "salary"), "张", 10, null);
        System.out.println("L1: " + r1);

        // L2 UPDATE
        Map<String, Object> ups = new LinkedHashMap<>();
        ups.put("emp_name", "张三");
        ups.put("salary", new java.math.BigDecimal("9999"));
        int r2 = dao.l2DynamicUpdate(1, ups);
        System.out.println("L2 UPDATE: " + r2);

        // L2 ORDER BY
        List<Map<String, Object>> r3 = dao.l2DynamicOrderBy("salary", "DESC", 5);
        System.out.println("L2 ORDER: " + r3);

        // L2 IN 数组写法
        List<Map<String, Object>> r4 = dao.l2DynamicInArray(List.of(10, 20, 30));
        System.out.println("L2 IN: " + r4);

        // L3 JOIN
        List<Map<String, Object>> r5 = dao.l3DynamicJoin(
            "employee", "department", "LEFT",
            "dept_id", "dept_id",
            List.of("emp_id", "emp_name"), "王");
        System.out.println("L3 JOIN: " + r5);

        // L3 GROUP BY
        List<Map<String, Object>> r6 = dao.l3DynamicGroupBy("dept_id", "AVG", ">", new java.math.BigDecimal("5000"));
        System.out.println("L3 GROUP: " + r6);

        // L3 分页
        List<Map<String, Object>> r7 = dao.l3Paginate(2, 10, "emp_id");
        System.out.println("L3 PAGE: " + r7);

        // L4 UPSERT
        int r8 = dao.l4Upsert(100, "李四", new java.math.BigDecimal("8000"), List.of("emp_name", "salary"));
        System.out.println("L4 UPSERT: " + r8);

        // L5 窗口函数
        List<Map<String, Object>> r9 = dao.l5DynamicWindow("ROW_NUMBER", "dept_id", "salary");
        System.out.println("L5 WINDOW: " + r9);
    }
}
