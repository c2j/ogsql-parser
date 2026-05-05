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
        let mut visited = HashSet::new();
        stmt.body = resolve_node(&stmt.body, &fragment_map, &mut visited)?;
    }

    let mut resolved_fragments = mapper.fragments.clone();
    for frag in &mut resolved_fragments {
        let mut visited = HashSet::new();
        frag.body = resolve_node(&frag.body, &fragment_map, &mut visited)?;
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
    visited: &mut HashSet<String>,
) -> Result<SqlNode, IbatisError> {
    match node {
        SqlNode::Include { refid } => {
            if visited.contains(refid) {
                let mut chain: Vec<String> = visited.iter().cloned().collect();
                chain.push(refid.clone());
                return Err(IbatisError::CircularInclude { chain });
            }

            let fragment_body = fragments
                .get(refid.as_str())
                .ok_or_else(|| IbatisError::UnknownFragment {
                    refid: refid.clone(),
                })?;

            visited.insert(refid.clone());
            let resolved = resolve_node(fragment_body, fragments, visited)?;
            visited.remove(refid);

            Ok(resolved)
        }
        SqlNode::If { test, children } => {
            let resolved_children = resolve_children(children, fragments, visited)?;
            Ok(SqlNode::If {
                test: test.clone(),
                children: resolved_children,
            })
        }
        SqlNode::Choose { branches } => {
            let mut resolved_branches = Vec::new();
            for (test, children) in branches {
                let resolved_children = resolve_children(children, fragments, visited)?;
                resolved_branches.push((test.clone(), resolved_children));
            }
            Ok(SqlNode::Choose {
                branches: resolved_branches,
            })
        }
        SqlNode::Where { children } => {
            let resolved_children = resolve_children(children, fragments, visited)?;
            Ok(SqlNode::Where {
                children: resolved_children,
            })
        }
        SqlNode::Set { children } => {
            let resolved_children = resolve_children(children, fragments, visited)?;
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
            let resolved_children = resolve_children(children, fragments, visited)?;
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
            let resolved_children = resolve_children(children, fragments, visited)?;
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
            let resolved_children = resolve_children(children, fragments, visited)?;
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
        .map(|c| resolve_node(c, fragments, visited))
        .collect()
}
