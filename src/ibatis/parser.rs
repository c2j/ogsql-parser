//! iBatis XML mapper 文件解析器。
//!
//! 使用 quick-xml 流式 event-based API 解析 XML。
//! 关键: trim_text 始终为 false — SQL 空白有意义。

use quick_xml::events::Event;
use quick_xml::Reader;

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{MapperFile, MapperStatement, ParameterMapDef, ParameterMapEntry, SqlFragment, SqlNode, StatementKind};
use crate::ibatis::util::{find_closing_brace, parse_param_type};

const SKIP_TAGS: &[&str] = &["resultMap", "cache", "cache-ref", "selectKey"];

pub fn parse_xml(xml: &[u8]) -> Result<MapperFile, IbatisError> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);

    let mut buf = Vec::new();
    let mut namespace = String::new();
    let mut fragments: Vec<SqlFragment> = Vec::new();
    let mut parameter_maps: Vec<ParameterMapDef> = Vec::new();
    let mut statements: Vec<MapperStatement> = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = e.local_name();
                if tag.as_ref().eq_ignore_ascii_case(b"mapper")
                    || tag.as_ref().eq_ignore_ascii_case(b"sqlMap")
                {
                    namespace = get_attr(&e, "namespace").unwrap_or_default();
                } else if tag.as_ref().eq_ignore_ascii_case(b"sql") {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    let children = read_node_tree(&mut reader, b"sql");
                    fragments.push(SqlFragment {
                        id,
                        body: merge_children(children),
                    });
                } else if let Some(kind) = statement_kind(tag.as_ref()) {
                    let line = byte_offset_to_line(xml, reader.buffer_position() as usize);
                    let id = get_attr(&e, "id").unwrap_or_default();
                    let parameter_type = get_attr(&e, "parameterType")
                        .or_else(|| get_attr(&e, "parameterClass"))
                        .or_else(|| get_attr(&e, "parameterMap"));
                    let result_type = get_attr(&e, "resultType")
                        .or_else(|| get_attr(&e, "resultMap"))
                        .or_else(|| get_attr(&e, "resultClass"));
                    let children = read_node_tree(&mut reader, tag.as_ref());
                    statements.push(MapperStatement {
                        kind,
                        id,
                        parameter_type,
                        result_type,
                        body: merge_children(children),
                        line,
                    });
                } else if tag.as_ref().eq_ignore_ascii_case(b"parameterMap") {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    let class = get_attr(&e, "class");
                    let entries = parse_parameter_map_entries(&mut reader);
                    parameter_maps.push(ParameterMapDef { id, class, params: entries });
                } else if is_skip_tag(tag.as_ref()) {
                    skip_content(&mut reader, tag.as_ref());
                }
            }
            Ok(Event::Empty(e)) => {
                let tag = e.local_name();
                if tag.as_ref().eq_ignore_ascii_case(b"sql") {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    fragments.push(SqlFragment {
                        id,
                        body: SqlNode::Text {
                            content: String::new(),
                        },
                    });
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => {
                return Err(IbatisError::XmlError {
                    line: byte_offset_to_line(xml, reader.error_position() as usize),
                    message: e.to_string(),
                });
            }
            _ => {}
        }
    }

    Ok(MapperFile {
        namespace,
        fragments,
        parameter_maps,
        statements,
    })
}

fn statement_kind(tag: &[u8]) -> Option<StatementKind> {
    if tag.eq_ignore_ascii_case(b"select") {
        Some(StatementKind::Select)
    } else if tag.eq_ignore_ascii_case(b"insert") {
        Some(StatementKind::Insert)
    } else if tag.eq_ignore_ascii_case(b"update") {
        Some(StatementKind::Update)
    } else if tag.eq_ignore_ascii_case(b"delete") {
        Some(StatementKind::Delete)
    } else {
        None
    }
}

fn is_skip_tag(tag: &[u8]) -> bool {
    SKIP_TAGS
        .iter()
        .any(|t| tag.eq_ignore_ascii_case(t.as_bytes()))
}

fn read_node_tree(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> Vec<SqlNode> {
    let mut nodes = Vec::new();
    let mut text_buf = String::new();
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                text_buf.push_str(&String::from_utf8_lossy(&e));
            }
            Ok(Event::GeneralRef(e)) => {
                let name = String::from_utf8_lossy(&e).into_owned();
                match name.as_str() {
                    "lt" => text_buf.push('<'),
                    "gt" => text_buf.push('>'),
                    "amp" => text_buf.push('&'),
                    "apos" => text_buf.push('\''),
                    "quot" => text_buf.push('"'),
                    other => {
                        text_buf.push('&');
                        text_buf.push_str(other);
                        text_buf.push(';');
                    }
                };
            }
            Ok(Event::CData(e)) => {
                let raw = String::from_utf8_lossy(&e);
                text_buf.push_str(&decode_xml_entities(&raw));
            }
            Ok(Event::Start(e)) => {
                flush_text_to_nodes(&mut text_buf, &mut nodes);
                let ln = e.local_name();
                if is_skip_tag(ln.as_ref()) {
                    skip_content(reader, ln.as_ref());
                } else if let Some(node) = parse_dynamic_element(reader, ln.as_ref(), &e) {
                    nodes.push(node);
                } else {
                    let tag_name = String::from_utf8_lossy(ln.as_ref());
                    let attrs = format_attributes(&e);
                    let inner = read_node_tree(reader, ln.as_ref());
                    let mut text = format!("<{}{}>", tag_name, attrs);
                    for child in &inner {
                        text.push_str(&simple_node_to_text(child));
                    }
                    text.push_str(&format!("</{}>", tag_name));
                    text_buf.push_str(&text);
                    flush_text_to_nodes(&mut text_buf, &mut nodes);
                }
            }
            Ok(Event::Empty(e)) => {
                flush_text_to_nodes(&mut text_buf, &mut nodes);
                let ln = e.local_name();
                if ln.as_ref().eq_ignore_ascii_case(b"include") {
                    if let Some(refid) = get_attr(&e, "refid") {
                        nodes.push(SqlNode::Include { refid });
                    }
                } else if ln.as_ref().eq_ignore_ascii_case(b"bind") {
                    nodes.push(SqlNode::Bind {
                        name: get_attr(&e, "name").unwrap_or_default(),
                        value: get_attr(&e, "value").unwrap_or_default(),
                    });
                } else {
                    let tag_name = String::from_utf8_lossy(ln.as_ref());
                    let attrs = format_attributes(&e);
                    text_buf.push_str(&format!("<{}{}/>", tag_name, attrs));
                }
            }
            Ok(Event::End(e)) => {
                let ln = e.local_name();
                if ln.as_ref().eq_ignore_ascii_case(end_tag) {
                    flush_text_to_nodes(&mut text_buf, &mut nodes);
                    break;
                }
            }
            Ok(Event::Eof) => {
                flush_text_to_nodes(&mut text_buf, &mut nodes);
                break;
            }
            _ => {}
        }
    }
    nodes
}

fn flush_text_to_nodes(text_buf: &mut String, nodes: &mut Vec<SqlNode>) {
    if text_buf.is_empty() {
        return;
    }
    let text = std::mem::take(text_buf);
    nodes.extend(parse_text_to_nodes(&text));
}

fn parse_text_to_nodes(text: &str) -> Vec<SqlNode> {
    let mut nodes = Vec::new();
    let mut current_text = String::new();
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        if chars[i] == '#' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                if !current_text.is_empty() {
                    nodes.push(SqlNode::Text {
                        content: std::mem::take(&mut current_text),
                    });
                }
                let param: String = chars[i + 2..end].iter().collect();
                let (name, java_type) = parse_param_type(&param);
                nodes.push(SqlNode::Parameter { name, java_type });
                i = end + 1;
                continue;
            }
        }
        if chars[i] == '$' && i + 1 < len && chars[i + 1] == '{' {
            if let Some(end) = find_closing_brace(&chars, i + 2) {
                if !current_text.is_empty() {
                    nodes.push(SqlNode::Text {
                        content: std::mem::take(&mut current_text),
                    });
                }
                let raw: String = chars[i + 2..end].iter().collect();
                let (expr, java_type) = parse_param_type(&raw);
                nodes.push(SqlNode::RawExpr { expr, java_type });
                i = end + 1;
                continue;
            }
        }

        // iBatis 2.x #param# format
        if chars[i] == '#' && (i + 1 >= len || chars[i + 1] != '{') {
            let start = i + 1;
            let mut end = start;
            while end < len && chars[end] != '#' {
                end += 1;
            }
            if end < len && end > start {
                let param: String = chars[start..end].iter().collect();
                if !param.contains(' ') && !param.contains('\n') && !param.contains('\r') {
                    if !current_text.is_empty() {
                        nodes.push(SqlNode::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    let (name, java_type) = parse_param_type(&param);
                    nodes.push(SqlNode::Parameter { name, java_type });
                    i = end + 1;
                    continue;
                }
            }
        }

        // iBatis 2.x $param$ format
        if chars[i] == '$' && (i + 1 >= len || chars[i + 1] != '{') {
            let start = i + 1;
            let mut end = start;
            while end < len && chars[end] != '$' {
                end += 1;
            }
            if end < len && end > start {
                let param: String = chars[start..end].iter().collect();
                if !param.contains(' ') && !param.contains('\n') && !param.contains('\r') {
                    if !current_text.is_empty() {
                        nodes.push(SqlNode::Text {
                            content: std::mem::take(&mut current_text),
                        });
                    }
                    let raw: String = chars[start..end].iter().collect();
                    let (expr, java_type) = parse_param_type(&raw);
                    nodes.push(SqlNode::RawExpr { expr, java_type });
                    i = end + 1;
                    continue;
                }
            }
        }
        current_text.push(chars[i]);
        i += 1;
    }
    if !current_text.is_empty() {
        nodes.push(SqlNode::Text {
            content: current_text,
        });
    }
    nodes
}

fn parse_dynamic_element(
    reader: &mut Reader<&[u8]>,
    tag: &[u8],
    element: &quick_xml::events::BytesStart<'_>,
) -> Option<SqlNode> {
    if tag.eq_ignore_ascii_case(b"if") {
        let test = get_attr(element, "test").unwrap_or_default();
        let children = read_node_tree(reader, b"if");
        Some(SqlNode::If { test, prepend: get_attr(element, "prepend"), children })
    } else if tag.eq_ignore_ascii_case(b"where") {
        let children = read_node_tree(reader, b"where");
        Some(SqlNode::Where { children })
    } else if tag.eq_ignore_ascii_case(b"set") {
        let children = read_node_tree(reader, b"set");
        Some(SqlNode::Set { children })
    } else if tag.eq_ignore_ascii_case(b"trim") {
        let children = read_node_tree(reader, b"trim");
        Some(SqlNode::Trim {
            prefix: get_attr(element, "prefix"),
            suffix: get_attr(element, "suffix"),
            prefix_overrides: get_attr(element, "prefixOverrides"),
            suffix_overrides: get_attr(element, "suffixOverrides"),
            children,
        })
    } else if tag.eq_ignore_ascii_case(b"foreach") {
        let children = read_node_tree(reader, b"foreach");
        Some(SqlNode::ForEach {
            collection: get_attr(element, "collection").unwrap_or_default(),
            item: get_attr(element, "item").unwrap_or_else(|| "item".to_string()),
            index: get_attr(element, "index"),
            open: get_attr(element, "open"),
            separator: get_attr(element, "separator"),
            close: get_attr(element, "close"),
            prepend: get_attr(element, "prepend"),
            children,
        })
    } else if tag.eq_ignore_ascii_case(b"choose") {
        let children = read_node_tree(reader, b"choose");
        let branches = parse_choose_branches(children);
        Some(SqlNode::Choose { branches })
    } else if tag.eq_ignore_ascii_case(b"when") {
        let test = get_attr(element, "test").unwrap_or_default();
        let children = read_node_tree(reader, b"when");
        Some(SqlNode::If { test, prepend: get_attr(element, "prepend"), children })
    } else if tag.eq_ignore_ascii_case(b"otherwise") {
        let children = read_node_tree(reader, b"otherwise");
        Some(SqlNode::If {
            test: String::new(),
            prepend: get_attr(element, "prepend"),
            children,
        })
    } else if tag.eq_ignore_ascii_case(b"dynamic") {
        let children = read_node_tree(reader, b"dynamic");
        let prepend = get_attr(element, "prepend");
        match prepend {
            Some(p) if !p.is_empty() => Some(SqlNode::Trim {
                prefix: Some(p),
                suffix: None,
                prefix_overrides: Some("AND |OR ".to_string()),
                suffix_overrides: None,
                children,
            }),
            _ => Some(SqlNode::Sequence { children }),
        }
    } else if tag.eq_ignore_ascii_case(b"iterate") {
        let children = read_node_tree(reader, b"iterate");
        Some(SqlNode::ForEach {
            collection: get_attr(element, "property").unwrap_or_default(),
            item: String::new(),
            index: None,
            open: get_attr(element, "open"),
            separator: get_attr(element, "conjunction"),
            close: get_attr(element, "close"),
            prepend: get_attr(element, "prepend"),
            children,
        })
    } else if is_ibatis2_conditional(tag) {
        let test = synthesize_ibatis2_test(element);
        let prepend = get_attr(element, "prepend");
        let children = read_node_tree(reader, tag);
        Some(SqlNode::If { test, prepend, children })
    } else {
        None
    }
}

fn parse_choose_branches(children: Vec<SqlNode>) -> Vec<(Option<String>, Vec<SqlNode>)> {
    let mut branches = Vec::new();
    for child in children {
        match child {
            SqlNode::If { test, prepend: _, children } => {
                if test.is_empty() {
                    branches.push((None, children));
                } else {
                    branches.push((Some(test), children));
                }
            }
            other => {
                if let Some(last) = branches.last_mut() {
                    last.1.push(other);
                }
            }
        }
    }
    branches
}

fn merge_children(children: Vec<SqlNode>) -> SqlNode {
    match children.len() {
        0 => SqlNode::Text {
            content: String::new(),
        },
        1 => children.into_iter().next().unwrap(),
        _ => SqlNode::Sequence { children },
    }
}

fn simple_node_to_text(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => content.clone(),
        SqlNode::Parameter { name, java_type } => match java_type {
            Some(t) => format!("#{{{},{}}}", name, format!("javaType={}", t)),
            None => format!("#{{{}}}", name),
        },
        SqlNode::RawExpr { expr, java_type } => match java_type {
            Some(t) => format!("${{{},{}}}", expr, format!("javaType={}", t)),
            None => format!("${{{}}}", expr),
        },
        _ => String::new(),
    }
}

fn skip_content(reader: &mut Reader<&[u8]>, end_tag: &[u8]) {
    let mut depth: u32 = 1;
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => depth += 1,
            Ok(Event::End(e)) => {
                depth -= 1;
                if depth == 0 && e.local_name().as_ref().eq_ignore_ascii_case(end_tag) {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }
}

fn parse_parameter_map_entries(reader: &mut Reader<&[u8]>) -> Vec<ParameterMapEntry> {
    let mut entries = Vec::new();
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Empty(e)) => {
                if e.local_name().as_ref().eq_ignore_ascii_case(b"parameter") {
                    entries.push(ParameterMapEntry {
                        property: get_attr(&e, "property").unwrap_or_default(),
                        jdbc_type: get_attr(&e, "jdbcType"),
                        java_type: get_attr(&e, "javaType"),
                    });
                }
            }
            Ok(Event::Start(e)) => {
                if e.local_name().as_ref().eq_ignore_ascii_case(b"parameter") {
                    entries.push(ParameterMapEntry {
                        property: get_attr(&e, "property").unwrap_or_default(),
                        jdbc_type: get_attr(&e, "jdbcType"),
                        java_type: get_attr(&e, "javaType"),
                    });
                    skip_content(reader, b"parameter");
                }
            }
            Ok(Event::End(e)) => {
                if e.local_name().as_ref().eq_ignore_ascii_case(b"parameterMap") {
                    break;
                }
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }
    entries
}

fn get_attr(element: &quick_xml::events::BytesStart<'_>, name: &str) -> Option<String> {
    element.attributes().find_map(|a| {
        a.ok().and_then(|a| {
            if a.key
                .local_name()
                .as_ref()
                .eq_ignore_ascii_case(name.as_bytes())
            {
                Some(String::from_utf8_lossy(&a.value).into_owned())
            } else {
                None
            }
        })
    })
}

fn format_attributes(element: &quick_xml::events::BytesStart<'_>) -> String {
    let mut result = String::new();
    for attr in element.attributes().flatten() {
        let ln = attr.key.local_name();
        let key = String::from_utf8_lossy(ln.as_ref());
        let value = String::from_utf8_lossy(&attr.value);
        result.push_str(&format!(" {}=\"{}\"", key, value));
    }
    result
}

fn decode_xml_entities(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        if c == '&' {
            let rest = &s[i..];
            if let Some(entity) = rest.find(';') {
                let name = &rest[1..entity];
                let decoded = match name {
                    "lt" => Some('<'),
                    "gt" => Some('>'),
                    "amp" => Some('&'),
                    "apos" => Some('\''),
                    "quot" => Some('"'),
                    _ => None,
                };
                if let Some(ch) = decoded {
                    result.push(ch);
                    for _ in 0..entity {
                        chars.next();
                    }
                    continue;
                }
            }
            result.push(c);
        } else {
            result.push(c);
        }
    }
    result
}

fn byte_offset_to_line(source: &[u8], offset: usize) -> usize {
    let mut line = 1;
    for &b in &source[..offset.min(source.len())] {
        if b == b'\n' {
            line += 1;
        }
    }
    line
}

fn is_ibatis2_conditional(tag: &[u8]) -> bool {
    const TAGS: &[&[u8]] = &[
        b"isEqual",
        b"isNotEqual",
        b"isGreaterThan",
        b"isGreaterEqual",
        b"isLessThan",
        b"isLessEqual",
        b"isNotNull",
        b"isNotEmpty",
        b"isParameterPresent",
        b"isNotParameterPresent",
        b"isPropertyAvailable",
        b"isNotPropertyAvailable",
    ];
    TAGS.iter()
        .any(|t| tag.eq_ignore_ascii_case(t))
}

fn synthesize_ibatis2_test(element: &quick_xml::events::BytesStart<'_>) -> String {
    let property = get_attr(element, "property").unwrap_or_default();
    let compare_value = get_attr(element, "compareValue");
    match compare_value {
        Some(cv) => format!("{} == '{}'", property, cv),
        None => {
            if property.is_empty() {
                "true".to_string()
            } else {
                format!("{} != null", property)
            }
        }
    }
}
