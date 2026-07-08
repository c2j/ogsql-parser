// description: ORDER BY positional reference with GROUP BY
// nowarn: R012
// statements: 1

public interface PositionalRefOk {
    @Select("SELECT col1, COUNT(*) FROM t1 GROUP BY col1 ORDER BY 1")
    java.util.List<String> find();
}
