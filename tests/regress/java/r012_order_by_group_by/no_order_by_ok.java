// description: GROUP BY without ORDER BY, no R012
// nowarn: R012
// statements: 1

public interface NoOrderByOk {
    @Select("SELECT col1, COUNT(*) FROM t1 GROUP BY col1")
    java.util.List<String> find();
}
