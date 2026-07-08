// description: ORDER BY positional reference with DISTINCT
// nowarn: R011
// statements: 1

public interface PositionalRefOk {
    @Select("SELECT DISTINCT col1, col2 FROM t1 ORDER BY 1")
    java.util.List<String> find();
}
