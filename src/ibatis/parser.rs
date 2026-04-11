//! iBatis XML mapper 文件解析器。
//!
//! 使用 quick-xml 流式 event-based API 解析 XML。
//! 关键: trim_text 始终为 false — SQL 空白有意义。

use quick_xml::escape::unescape;
use quick_xml::events::Event;
use quick_xml::Reader;

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{MapperFile, MapperStatement, SqlFragment, SqlNode, StatementKind};

const SKIP_TAGS: &[&str] = &["resultMap", "cache", "cache-ref", "parameterMap"];

pub fn parse_xml(xml: &[u8]) -> Result<MapperFile, IbatisError> {
    let mut reader = Reader::from_reader(xml);
    reader.config_mut().trim_text(false);

    let mut buf = Vec::new();
    let mut namespace = String::new();
    let mut fragments: Vec<SqlFragment> = Vec::new();
    let mut statements: Vec<MapperStatement> = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = e.local_name();
                if tag.as_ref().eq_ignore_ascii_case(b"mapper") {
                    namespace = get_attr(&e, "namespace").unwrap_or_default();
                } else if tag.as_ref().eq_ignore_ascii_case(b"sql") {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    let body_text = read_text_content(&mut reader, b"sql");
                    fragments.push(SqlFragment {
                        id,
                        body: SqlNode::Text { content: body_text },
                    });
                } else if let Some(kind) = statement_kind(tag.as_ref()) {
                    let id = get_attr(&e, "id").unwrap_or_default();
                    let parameter_type = get_attr(&e, "parameterType");
                    let result_type =
                        get_attr(&e, "resultType").or_else(|| get_attr(&e, "resultMap"));
                    let body_text = read_text_content(&mut reader, tag.as_ref());
                    statements.push(MapperStatement {
                        kind,
                        id,
                        parameter_type,
                        result_type,
                        body: SqlNode::Text { content: body_text },
                    });
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

fn read_text_content(reader: &mut Reader<&[u8]>, end_tag: &[u8]) -> String {
    let mut content = String::new();
    let mut depth: u32 = 1;
    let mut buf = Vec::new();

    loop {
        buf.clear();
        match reader.read_event_into(&mut buf) {
            Ok(Event::Text(e)) => {
                let raw = String::from_utf8_lossy(&e).into_owned();
                let text = match unescape(&raw) {
                    Ok(cow) => cow.into_owned(),
                    Err(_) => raw,
                };
                content.push_str(&text);
            }
            Ok(Event::CData(e)) => {
                content.push_str(&String::from_utf8_lossy(&e));
            }
            Ok(Event::Start(e)) => {
                depth += 1;
                let ln = e.local_name();
                let tag_name = String::from_utf8_lossy(ln.as_ref());
                let attrs = format_attributes(&e);
                content.push_str(&format!("<{}{}>", tag_name, attrs));
            }
            Ok(Event::End(e)) => {
                depth -= 1;
                let ln = e.local_name();
                if depth == 0 && ln.as_ref().eq_ignore_ascii_case(end_tag) {
                    break;
                }
                let tag_name = String::from_utf8_lossy(ln.as_ref());
                content.push_str(&format!("</{}>", tag_name));
            }
            Ok(Event::Empty(e)) => {
                let ln = e.local_name();
                let tag_name = String::from_utf8_lossy(ln.as_ref());
                let attrs = format_attributes(&e);
                content.push_str(&format!("<{}{}/>", tag_name, attrs));
            }
            Ok(Event::Eof) => break,
            _ => {}
        }
    }
    content
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

fn byte_offset_to_line(source: &[u8], offset: usize) -> usize {
    let mut line = 1;
    for &b in &source[..offset.min(source.len())] {
        if b == b'\n' {
            line += 1;
        }
    }
    line
}
