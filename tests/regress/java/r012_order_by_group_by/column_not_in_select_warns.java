// description: ORDER BY column not in SELECT with GROUP BY
// warn: R012
// statements: 1

public interface ColumnNotInSelectWarns {
    @Select("SELECT col1, COUNT(*) FROM t1 GROUP BY col1 ORDER BY col2")
    java.util.List<String> find();
}
