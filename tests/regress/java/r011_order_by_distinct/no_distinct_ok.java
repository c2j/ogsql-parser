// description: ORDER BY without DISTINCT, no R011
// nowarn: R011
// statements: 1

public interface NoDistinctOk {
    @Select("SELECT col1 FROM t1 ORDER BY col2")
    java.util.List<String> find();
}
