// description: SELECT * with DISTINCT is skipped
// nowarn: R011
// statements: 1

public interface SelectStarSkipped {
    @Select("SELECT DISTINCT * FROM t1 ORDER BY col1")
    java.util.List<String> find();
}
