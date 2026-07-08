// description: ORDER BY aliased column in SELECT with DISTINCT
// nowarn: R011
// statements: 1

public interface AliasInSelectOk {
    @Select("SELECT DISTINCT col1 AS c1 FROM t1 ORDER BY c1")
    java.util.List<String> find();
}
