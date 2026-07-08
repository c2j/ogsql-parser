// description: ORDER BY aliased column in SELECT with GROUP BY
// nowarn: R012
// statements: 1

public interface AliasInSelectOk {
    @Select("SELECT col1 AS c1, COUNT(*) FROM t1 GROUP BY col1 ORDER BY c1")
    java.util.List<String> find();
}
