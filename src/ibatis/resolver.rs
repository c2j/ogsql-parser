//! <include> 片段解析。
//!
//! 解析 <sql id="..."> 片段引用，内联展开到引用位置。
//! v1 仅支持同文件内的片段引用。

use std::collections::{HashMap, HashSet};

use crate::ibatis::error::IbatisError;
use crate::ibatis::types::{MapperFile, SqlNode};

pub fn resolve_includes(mapper: &MapperFile) -> Result<MapperFile, IbatisError> {
    let fragment_map: HashMap<&str, &SqlNode> = mapper
        .fragments
        .iter()
        .map(|f| (f.id.as_str(), &f.body))
        .collect();

    let mut resolved_statements = mapper.statements.clone();
    for stmt in &mut resolved_statements {
        let visited = HashSet::new();
        stmt.body = resolve_node(&stmt.body, &fragment_map, visited)?;
    }

    let mut resolved_fragments = mapper.fragments.clone();
    for frag in &mut resolved_fragments {
        let visited = HashSet::new();
        frag.body = resolve_node(&frag.body, &fragment_map, visited)?;
    }

    Ok(MapperFile {
        namespace: mapper.namespace.clone(),
        fragments: resolved_fragments,
        statements: resolved_statements,
    })
}

fn resolve_node(
    node: &SqlNode,
    fragments: &HashMap<&str, &SqlNode>,
    mut visited: HashSet<String>,
) -> Result<SqlNode, IbatisError> {
    match node {
        SqlNode::Text { content } => {
            if content.contains("<include") {
                let resolved = resolve_includes_in_text(content, fragments, &mut visited)?;
                Ok(SqlNode::Text { content: resolved })
            } else {
                Ok(node.clone())
            }
        }
        SqlNode::If { test, children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::If {
                test: test.clone(),
                children: resolved_children,
            })
        }
        SqlNode::Choose { branches } => {
            let mut resolved_branches = Vec::new();
            for (test, children) in branches {
                let resolved_children = resolve_children(children, fragments, &mut visited)?;
                resolved_branches.push((test.clone(), resolved_children));
            }
            Ok(SqlNode::Choose {
                branches: resolved_branches,
            })
        }
        SqlNode::Where { children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::Where {
                children: resolved_children,
            })
        }
        SqlNode::Set { children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::Set {
                children: resolved_children,
            })
        }
        SqlNode::Trim {
            prefix,
            suffix,
            prefix_overrides,
            suffix_overrides,
            children,
        } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::Trim {
                prefix: prefix.clone(),
                suffix: suffix.clone(),
                prefix_overrides: prefix_overrides.clone(),
                suffix_overrides: suffix_overrides.clone(),
                children: resolved_children,
            })
        }
        SqlNode::ForEach {
            collection,
            item,
            index,
            open,
            separator,
            close,
            children,
        } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::ForEach {
                collection: collection.clone(),
                item: item.clone(),
                index: index.clone(),
                open: open.clone(),
                separator: separator.clone(),
                close: close.clone(),
                children: resolved_children,
            })
        }
        SqlNode::Sequence { children } => {
            let resolved_children = resolve_children(children, fragments, &mut visited)?;
            Ok(SqlNode::Sequence {
                children: resolved_children,
            })
        }
        other => Ok(other.clone()),
    }
}

fn resolve_children(
    children: &[SqlNode],
    fragments: &HashMap<&str, &SqlNode>,
    visited: &mut HashSet<String>,
) -> Result<Vec<SqlNode>, IbatisError> {
    children
        .iter()
        .map(|c| resolve_node(c, fragments, visited.clone()))
        .collect()
}

fn resolve_includes_in_text(
    text: &str,
    fragments: &HashMap<&str, &SqlNode>,
    visited: &mut HashSet<String>,
) -> Result<String, IbatisError> {
    let mut result = String::with_capacity(text.len());
    let mut pos = 0;

    while pos < text.len() {
        if let Some(start) = text[pos..].find("<include") {
            let abs_start = pos + start;
            result.push_str(&text[pos..abs_start]);

            let rest = &text[abs_start..];
            if let Some(refid) = extract_refid(rest) {
                if visited.contains(&refid) {
                    let mut chain: Vec<String> = visited.iter().cloned().collect();
                    chain.push(refid.clone());
                    return Err(IbatisError::CircularInclude { chain });
                }

                let fragment_body =
                    fragments
                        .get(refid.as_str())
                        .ok_or_else(|| IbatisError::UnknownFragment {
                            refid: refid.clone(),
                        })?;

                visited.insert(refid.clone());
                let expanded_text = node_to_flat_text(fragment_body);
                let expanded = resolve_includes_in_text(&expanded_text, fragments, visited)?;
                visited.remove(&refid);

                result.push_str(&expanded);

                let tag_end = if let Some(e) = rest.find("/>") {
                    abs_start + e + 2
                } else if let Some(e) = rest.find('>') {
                    abs_start + e + 1
                } else {
                    text.len()
                };
                pos = tag_end;
            } else {
                result.push_str("<include");
                pos = abs_start + 8;
            }
        } else {
            result.push_str(&text[pos..]);
            break;
        }
    }

    Ok(result)
}

fn extract_refid(tag_text: &str) -> Option<String> {
    let patterns = ["refid=\"", "refid='"];
    for pattern in patterns {
        if let Some(start) = tag_text.find(pattern) {
            let value_start = start + pattern.len();
            let quote_char = pattern.chars().last().unwrap();
            if let Some(end) = tag_text[value_start..].find(quote_char) {
                return Some(tag_text[value_start..value_start + end].to_string());
            }
        }
    }
    None
}

fn node_to_flat_text(node: &SqlNode) -> String {
    match node {
        SqlNode::Text { content } => content.clone(),
        SqlNode::Parameter { name } => format!("#{{{}}}", name),
        SqlNode::RawExpr { expr } => format!("${{{}}}", expr),
        SqlNode::If { children, .. } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Choose { branches } => branches
            .iter()
            .flat_map(|(_, children)| children.iter().map(node_to_flat_text))
            .collect(),
        SqlNode::Where { children } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Set { children } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Trim { children, .. } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::ForEach { children, .. } => children.iter().map(node_to_flat_text).collect(),
        SqlNode::Bind { .. } => String::new(),
        SqlNode::Sequence { children } => children.iter().map(node_to_flat_text).collect(),
    }
}
