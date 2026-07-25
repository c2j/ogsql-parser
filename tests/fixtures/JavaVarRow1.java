import java.sql.*;
import java.time.LocalDate;
import java.util.*;
import java.util.Date;

public class GaussDBDynamicSQL {

    // GaussDB/OpenGauss 连接串（与 PostgreSQL 兼容）
    private static final String URL = "jdbc:opengauss://localhost:26000/finance_db?currentSchema=public";
    private static final String USER = "dbuser";
    private static final String PASS = "dbpass";

    static {
        // OpenGauss 驱动；若用 GaussDB 商业版驱动，改为 com.huawei.gauss200.jdbc.Driver
        try {
            Class.forName("org.opengauss.Driver");
        } catch (ClassNotFoundException e) {
            throw new RuntimeException("缺少 GaussDB/OpenGauss JDBC 驱动", e);
        }
    }

    /**
     * 场景一：动态条件查询。
     * 根据传入的可选条件，动态拼接 WHERE 子句，所有值均使用 ? 占位。
     */
    public List<<Employee> searchEmployees(String col4, String name, Integer deptId, LocalDate hireDateAfter) throws SQLException {

        StringBuilder sql = new StringBuilder("SELECT emp_id, emp_name, dept_id, hire_date, ");
        sql.append(col4);          // 确保 col4 以空格开头，如 " ,salary"
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
        if (hireDateAfter != null) {
            sql.append("AND hire_date > ? ");
            params.add(Date.valueOf(hireDateAfter));
        }
        sql.append("ORDER BY emp_id");

        List<<Employee> result = new ArrayList<>();
        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql.toString())) {

            // 注入参数：索引从 1 开始
            // for (int i = 0; i < params.size(); i++) {
            //     ps.setObject(i + 1, params.get(i));
            // }
            ps.setString(1, params.get(0));
            ps.setInt(2,new Integer(params.get(1)));
            ps.setString(3, params.get(2));

            try (ResultSet rs = ps.executeQuery()) {
                while (rs.next()) {
                    result.add(new Employee(
                        rs.getInt("emp_id"),
                        rs.getString("emp_name"),
                        rs.getInt("dept_id"),
                        rs.getDate("hire_date"),
                        rs.getBigDecimal("salary")
                    ));
                }
            }
        }
        return result;
    }

    /**
     * 场景二：动态 INSERT。
     * 列名由代码内部白名单决定，VALUES 全部使用 ? 占位。
     */
    public int insertEmployee(Map<String, Object> columnValues) throws SQLException {
        // 白名单校验：只允许已知列，防止 SQL 注入
        Set<String> allowedCols = Set.of("emp_name", "dept_id", "hire_date", "salary");
        List<String> columns = new ArrayList<>();
        List<Object> values = new ArrayList<>();

        for (Map.Entry<String, Object> entry : columnValues.entrySet()) {
            if (!allowedCols.contains(entry.getKey())) {
                throw new IllegalArgumentException("非法列名: " + entry.getKey());
            }
            columns.add(entry.getKey());
            values.add(entry.getValue());
        }

        String colPart = String.join(", ", columns);
        String placeholderPart = String.join(", ", Collections.nCopies(columns.size(), "?"));
        String sql = "INSERT INTO employee (" + colPart + ") VALUES (" + placeholderPart + ")";

        try (Connection conn = DriverManager.getConnection(URL, USER, PASS);
             PreparedStatement ps = conn.prepareStatement(sql, Statement.RETURN_GENERATED_KEYS)) {

            for (int i = 0; i < values.size(); i++) {
                ps.setObject(i + 1, values.get(i));
            }

            int affected = ps.executeUpdate();

            // 若表定义了自增主键，可在此取回
            try (ResultSet rs = ps.getGeneratedKeys()) {
                if (rs.next()) {
                    System.out.println("生成主键: " + rs.getInt(1));
                }
            }
            return affected;
        }
    }

    // 简单实体
    public static class Employee {
        public final int empId;
        public final String empName;
        public final int deptId;
        public final Date hireDate;
        public final java.math.BigDecimal salary;
        // constructor ...
        public Employee(int empId, String empName, int deptId, Date hireDate, java.math.BigDecimal salary) {
            this.empId = empId; this.empName = empName; this.deptId = deptId;
            this.hireDate = hireDate; this.salary = salary;
        }
        @Override
        public String toString() {
            return String.format("Employee{id=%d, name=%s, dept=%d}", empId, empName, deptId);
        }
    }

    // 快速验证
    public static void main(String[] args) throws SQLException {
        GaussDBDynamicSQL dao = new GaussDBDynamicSQL();

        // 1. 动态查询：只传两个条件
        List<<Employee> list = dao.searchEmployees("张", 10, null);
        list.forEach(System.out::println);

        // 2. 动态插入：只插入部分列
        Map<String, Object> cols = new LinkedHashMap<>();
        cols.put("emp_name", "王小明");
        cols.put("dept_id", 10);
        cols.put("salary", new java.math.BigDecimal("8500.50"));
        dao.insertEmployee(cols);
    }
}
