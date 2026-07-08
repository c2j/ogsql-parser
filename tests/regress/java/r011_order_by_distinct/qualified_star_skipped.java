// description: t.* with DISTINCT is skipped
// nowarn: R011
// statements: 1

public interface QualifiedStarSkipped {
    @Select("SELECT DISTINCT t1.* FROM t1 ORDER BY col1")
    java.util.List<String> find();
}
