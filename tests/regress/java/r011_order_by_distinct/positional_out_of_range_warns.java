// description: ORDER BY positional reference out of range with DISTINCT
// warn: R011
// statements: 1

public interface PositionalOutOfRangeWarns {
    @Select("SELECT DISTINCT col1 FROM t1 ORDER BY 2")
    java.util.List<String> find();
}
