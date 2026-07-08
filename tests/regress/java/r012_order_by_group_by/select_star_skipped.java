// description: SELECT * with GROUP BY is skipped
// nowarn: R012
// statements: 1

public interface SelectStarSkipped {
    @Select("SELECT * FROM t1 GROUP BY col1 ORDER BY col2")
    java.util.List<String> find();
}
