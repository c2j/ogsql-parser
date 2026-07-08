// description: ORDER BY references a column in GROUP BY but not in SELECT
// nowarn: R012
// statements: 1

public interface GroupByColumnOk {
    @Select("SELECT a FROM t1 GROUP BY a, b ORDER BY b")
    java.util.List<String> find();
}
