// description: ORDER BY column not in SELECT with DISTINCT
// warn: R011
// statements: 1

public interface ColumnNotInSelectWarns {
    @Select("SELECT DISTINCT col1 FROM t1 ORDER BY col2")
    java.util.List<String> find();
}
