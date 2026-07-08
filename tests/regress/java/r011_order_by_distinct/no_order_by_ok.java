// description: DISTINCT without ORDER BY, no R011
// nowarn: R011
// statements: 1

public interface NoOrderByOk {
    @Select("SELECT DISTINCT col1 FROM t1")
    java.util.List<String> find();
}
