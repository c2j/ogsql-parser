use tree_sitter::Parser;

fn print_subtree(src: &str, node: tree_sitter::Node, depth: usize) {
    let text = &src[node.byte_range()];
    let text_short: String = text.chars().take(60).collect();
    println!(
        "{}{} [{},{} - {},{}] {}",
        "  ".repeat(depth),
        node.kind(),
        node.start_position().row,
        node.start_position().column,
        node.end_position().row,
        node.end_position().column,
        text_short.replace('\n', "\\n")
    );
    if !node.is_named() && text.len() < 40 && !text.contains('\n') {
        println!("{}-> \"{}\"", "  ".repeat(depth + 1), text);
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_subtree(src, child, depth + 1);
        }
    }
}

fn find_all<'a>(node: tree_sitter::Node<'a>, kind: &str, results: &mut Vec<tree_sitter::Node<'a>>) {
    if node.kind() == kind {
        results.push(node);
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            find_all(child, kind, results);
        }
    }
}

#[test]
fn print_ast() {
    let src = r#"
import java.util.*;
public class Test {
    void foo(Map<String, Object> columnValues) {
        Set<String> allowedCols = Set.of("emp_name", "dept_id", "hire_date", "salary");
        List<String> columns = new ArrayList<>();
        for (Map.Entry<String, Object> entry : columnValues.entrySet()) {
            if (!allowedCols.contains(entry.getKey())) throw new RuntimeException();
            columns.add(entry.getKey());
        }
        String placeholderPart = String.join(", ", Collections.nCopies(columns.size(), "?"));
        String sql = "INSERT INTO employee (" + placeholderPart + ") VALUES (" + placeholderPart + ")";
    }
}
"#;
    let mut parser = Parser::new();
    parser.set_language(&tree_sitter_java::LANGUAGE.into()).unwrap();
    let tree = parser.parse(src, None).unwrap();
    let root = tree.root_node();

    println!("=== enhanced_for_statement ===");
    let mut for_nodes = Vec::new();
    find_all(root, "enhanced_for_statement", &mut for_nodes);
    for node in &for_nodes {
        print_subtree(src, *node, 0);
    }

    println!("\n=== method_invocations ===");
    let mut methods = Vec::new();
    find_all(root, "method_invocation", &mut methods);
    for (i, m) in methods.iter().enumerate() {
        let text = &src[m.byte_range()];
        let text_s: String = text.chars().take(80).collect();
        println!("\n--- [{}] {}", i, text_s.replace('\n', "\\n"));
        print_subtree(src, *m, 0);
    }

    println!("\n=== local_variable_declarations ===");
    let mut locals = Vec::new();
    find_all(root, "local_variable_declaration", &mut locals);
    for (i, l) in locals.iter().enumerate() {
        println!("\n--- [{}]", i);
        print_subtree(src, *l, 0);
    }
}
