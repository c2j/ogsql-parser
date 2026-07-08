// description: t.* with GROUP BY is skipped
// nowarn: R012
// statements: 1

public interface QualifiedStarSkipped {
    @Select("SELECT t1.* FROM t1 GROUP BY col1 ORDER BY col2")
    java.util.List<String> find();
}
