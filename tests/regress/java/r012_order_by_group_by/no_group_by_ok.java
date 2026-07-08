// description: ORDER BY without GROUP BY, no R012
// nowarn: R012
// statements: 1

public interface NoGroupByOk {
    @Select("SELECT col1 FROM t1 ORDER BY col2")
    java.util.List<String> find();
}
