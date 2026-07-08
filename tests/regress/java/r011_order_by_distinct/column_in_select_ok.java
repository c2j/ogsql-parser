// description: ORDER BY column is in SELECT list with DISTINCT
// nowarn: R011
// statements: 1

public interface ColumnInSelectOk {
    @Select("SELECT DISTINCT col1, col2 FROM t1 ORDER BY col1")
    java.util.List<String> find();
}
