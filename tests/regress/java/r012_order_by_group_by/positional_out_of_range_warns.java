// description: ORDER BY positional reference out of range with GROUP BY
// warn: R012
// statements: 1

public interface PositionalOutOfRangeWarns {
    @Select("SELECT col1, COUNT(*) FROM t1 GROUP BY col1 ORDER BY 3")
    java.util.List<String> find();
}
