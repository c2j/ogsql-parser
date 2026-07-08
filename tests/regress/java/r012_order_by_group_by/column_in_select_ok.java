// description: ORDER BY column is in SELECT list with GROUP BY
// nowarn: R012
// statements: 1

public interface ColumnInSelectOk {
    @Select("SELECT col1, COUNT(*) FROM t1 GROUP BY col1 ORDER BY col1")
    java.util.List<String> find();
}
