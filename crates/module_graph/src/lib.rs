use anyhow::Result;
use beans::{AstNode, Span};
use log::debug;
use napi_derive::napi;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use petgraph::algo::all_simple_paths;
use petgraph::algo::has_path_connecting;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::Direction;
use serde::Serialize;

use std::{
  collections::{HashMap, HashSet},
  path::PathBuf,
  sync::{Arc, Mutex},
};
use utils::{glob, GlobOptions, SemanticBuilder};

#[napi[object]]
#[derive(Debug, Clone)]
pub struct Options {
  pub cwd: Option<String>,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Dependency {
  pub from: String,
  pub to: String,
  pub ast_node: AstNode,
}

pub fn get_node(options: Option<Options>) -> Result<Vec<Dependency>> {
  let used = Arc::new(Mutex::new(Vec::new()));

  let alias = match &options {
    Some(Options {
      alias: Some(alias), ..
    }) => alias.clone(),
    _ => HashMap::new(),
  };

  let modules = match &options {
    Some(Options {
      modules: Some(modules),
      ..
    }) => modules.clone(),
    _ => vec!["node_modules".into(), "web_modules".into()],
  };

  let resolver_alias = alias
    .into_iter()
    .map(|(key, values)| {
      (key, values.into_iter().map(AliasValue::Path).collect())
    })
    .collect();

  let resolver = Resolver::new(ResolveOptions {
    alias: resolver_alias,
    modules,
    extensions: vec![
      ".ts".into(),
      ".js".into(),
      ".jsx".into(),
      ".tsx".into(),
      ".json".into(),
    ],
    ..ResolveOptions::default()
  });

  let handler = {
    let used = Arc::clone(&used);

    move |path: PathBuf| {
      debug!("path: {}", path.display().to_string());

      let mut inline_usages: Vec<Dependency> = Vec::new();

      SemanticBuilder::file(path.clone())
        .build_handler()
        .each_node(|handler, node| {
          if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
            let value = import_declaration.source.value.to_string();
            debug!("value: {}", value);

            if let Some(parent) = path.parent() {
              debug!("parent: {}", parent.display().to_string());
              let resolved = resolver.resolve(&parent, &value);
              if let Ok(resolved_path) = resolved {
                debug!(
                  "resolved_path: {}",
                  resolved_path.full_path().display().to_string()
                );

                let (span, loc) = handler.get_node_box(node);

                inline_usages.push(Dependency {
                  from: path.display().to_string(),
                  to: resolved_path.full_path().display().to_string(),
                  ast_node: AstNode {
                    span: Span {
                      start: span.start,
                      end: span.end,
                    },
                    loc: loc,
                  },
                });
              } else {
                eprintln!(
                  "no resolved path  {} in {}",
                  value,
                  path.display().to_string()
                );
              }
            } else {
              eprintln!("no parent path {}", path.display().to_string());
            }
          }
        });

      let mut used = used.lock().unwrap();
      used.extend(inline_usages);
    }
  };

  let op = if let Some(options) = options {
    Some(GlobOptions {
      cwd: options.cwd,
      pattern: options.pattern,
      ignore: options.ignore,
    })
  } else {
    None
  };

  glob(handler, op).map_err(|err| {
    napi::Error::new(napi::Status::GenericFailure, err.to_string())
  })?;

  let x = used.lock().unwrap().clone();

  Ok(x)
}

pub fn build_graph(
  used: &[Dependency],
  dir: Direction,
) -> (
  DiGraph<String, ()>,
  HashMap<(String, String), &Dependency>,
  HashMap<String, NodeIndex>,
) {
  let len = used.len();

  let mut graph = DiGraph::with_capacity(len, len);
  let mut module_map = HashMap::with_capacity(len);
  let mut node_indices = HashMap::with_capacity(len);

  for value in used.iter() {
    let from_node = *node_indices
      .entry(value.from.clone())
      .or_insert_with(|| graph.add_node(value.from.clone()));
    let to_node = *node_indices
      .entry(value.to.clone())
      .or_insert_with(|| graph.add_node(value.to.clone()));

    match dir {
      Direction::Outgoing => {
        module_map.insert((value.to.clone(), value.from.clone()), value);
        graph.add_edge(to_node, from_node, ());
      }
      Direction::Incoming => {
        module_map.insert((value.from.clone(), value.to.clone()), value);
        graph.add_edge(from_node, to_node, ());
      }
    }
  }

  (graph, module_map, node_indices)
}

pub fn get_dependents(
  file: String,
  options: Option<Options>,
) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;

  let (graph, module_map, node_indices) =
    build_graph(&used, Direction::Outgoing);

  let target_index = *node_indices.get(file.as_str()).unwrap();
  let mut result = Vec::new();

  // 使用 Dfs 遍历反向图
  let mut dfs = Dfs::new(&graph, target_index);

  let mut prev_node = target_index;

  while let Some(nx) = dfs.next(&graph) {
    if nx != target_index && !graph.contains_edge(prev_node, nx) {
      // 使用 all_simple_paths 找到所有路径
      for path in
        all_simple_paths::<Vec<_>, _>(&graph, target_index, prev_node, 0, None)
      {
        let inline_path = path
          .into_iter()
          .rev()
          .collect::<Vec<NodeIndex>>()
          .windows(2)
          .map(|window| {
            let source = graph[window[0]].clone();
            let target = graph[window[1]].clone();
            Cycle {
              source: source.to_string(),
              target: target.to_string(),
              ast_node: module_map[&(target, source)].ast_node.clone(),
            }
          })
          .collect();
        result.push(inline_path);
      }
    }
    prev_node = nx;
  }

  Ok(result)
}

#[derive(Debug, Clone)]
#[napi(object)]
pub struct DependencyNode {
  pub name: String,
  pub children: Vec<DependencyNode>,
  pub ast_node: Option<AstNode>,
}

pub fn get_dependencies(
  file: String,
  options: Option<Options>,
) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;
  let (graph, module_map, node_indices) =
    build_graph(&used, Direction::Incoming);

  let target_index = node_indices.get(file.as_str()).unwrap();
  let mut result = Vec::new();

  // 使用 Dfs 遍历图
  let mut dfs = Dfs::new(&graph, *target_index);
  while let Some(nx) = dfs.next(&graph) {
    if nx != *target_index {
      for path in petgraph::algo::all_simple_paths::<Vec<_>, _>(
        &graph,
        *target_index,
        nx,
        0,
        None,
      ) {
        let inline_path = path
          .windows(2)
          .map(|window| {
            let source = graph[window[0]].clone();
            let target = graph[window[1]].clone();
            Cycle {
              source: source.to_string(),
              target: target.to_string(),
              ast_node: module_map[&(source, target)].ast_node.clone(),
            }
          })
          .collect();
        result.push(inline_path);
      }

      if has_path_connecting(&graph, nx, *target_index, None) {
        println!("TODO cycle: {}", graph[nx].to_string());
      }
    }
  }

  Ok(result)
}

#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq)]
pub struct Cycle {
  pub source: String,
  pub target: String,
  pub ast_node: AstNode,
}

pub fn check_cycle(options: Option<Options>) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;

  let mut module_map = HashMap::new();
  let mut graph = DiGraph::new();
  let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();

  // 构建图的代码保持不变
  for value in used.iter() {
    let from = value.from.as_str();
    let to = value.to.as_str();

    let from_node =
      *node_map.entry(from).or_insert_with(|| graph.add_node(from));

    let to_node = *node_map.entry(to).or_insert_with(|| graph.add_node(to));

    module_map.insert((from, to), value);
    graph.add_edge(from_node, to_node, ());
  }

  // 使用 kosaraju_scc 算法找出强连通分量
  let sccs = kosaraju_scc(&graph);

  let mut result = Vec::new();

  // 遍历所有大小大于1的强连通分量
  for scc in sccs.into_iter().filter(|scc| scc.len() > 1) {
    let mut cycles = HashSet::new();
    let scc_set: HashSet<_> = scc.iter().cloned().collect();

    // 对SCC中的每个节点进行深度优先搜索
    for &start_node in &scc {
      // 创建一个新的DFS迭代器
      let mut dfs = Dfs::new(&graph, start_node);
      let mut path = Vec::new();
      let mut stack = Vec::new();

      // 使用DFS遍历图
      while let Some(nx) = dfs.next(&graph) {
        // 如果当前节点不在SCC中,跳过
        if !scc_set.contains(&nx) {
          continue;
        }

        // 更新路径
        while let Some(&last) = path.last() {
          if !graph.contains_edge(last, nx) {
            path.pop();
            stack.pop();
          } else {
            break;
          }
        }
        path.push(nx);
        stack.push(nx);

        // 检查当前节点的所有邻居
        for neighbor in graph.neighbors(nx) {
          if neighbor == start_node || stack.contains(&neighbor) {
            let cycle_start =
              stack.iter().position(|&n| n == neighbor).unwrap();
            let mut cycle = stack[cycle_start..]
              .windows(2)
              .map(|window| {
                let source = graph[window[0]];
                let target = graph[window[1]];
                Cycle {
                  source: source.to_string(),
                  target: target.to_string(),
                  ast_node: module_map[&(source, target)].ast_node.clone(),
                }
              })
              .collect::<Vec<Cycle>>();

            let last = stack.last().unwrap();
            cycle.push(Cycle {
              source: graph[*last].to_string(),
              target: graph[neighbor].to_string(),
              ast_node: module_map[&(graph[*last], graph[neighbor])]
                .ast_node
                .clone(),
            });

            // 标准化循环并添加到HashSet中
            let normalized_cycle = normalize_cycle(cycle);
            cycles.insert(normalized_cycle);
          }
        }
      }
    }

    // 如果在当前SCC中找到了循环,将它们添加到结果中
    if !cycles.is_empty() {
      result.extend(cycles);
    }
  }

  Ok(result)
}

fn normalize_cycle(mut cycle: Vec<Cycle>) -> Vec<Cycle> {
  if let Some(min_pos) = cycle
    .iter()
    .enumerate()
    .min_by_key(|(_, c)| &c.source)
    .map(|(i, _)| i)
  {
    cycle.rotate_left(min_pos);
  }
  cycle
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_danger_strings() {
    let mut alias = HashMap::new();

    // let x = &[
    //   ("@src", "src"),
    //   ("@public-component", "src/component/public-component"),
    //   ("@search-queries", "src/component/search-queries"),
    //   (
    //     "@shein-components/dateRangePicker2",
    //     "src/component/public-component/dateRangePicker2Wrapper",
    //   ),
    //   ("@", "src"),
    // ];

    let x = &[
      ("apis", "web_modules/apis"),
      ("common", "web_modules/common"),
      ("shein-lib", "web_modules/shein-lib"),
      ("hooks", "web_modules/hooks"),
      ("publicComponent", "web_modules/public/spmb"),
      ("@", "src"),
    ];

    for (key, value) in x.iter() {
      alias.insert(
        key.to_string(),
        vec![format!(
          "{}/{}",
          "/Users/ityuany/GitRepository/bmas",
          value.to_string()
        )],
      );
    }

    // alias.insert(
    //   "@src".to_string(),
    //   vec!["/Users/ityuany/GitRepository/wms/src".to_string()],
    // );
    // alias.insert(
    //   "@public-component".to_string(),
    //   vec![
    //     "/Users/ityuany/GitRepository/wms/src/component/public-component"
    //       .to_string(),
    //   ],
    // );
    // alias.insert(
    //   "@search-queries".to_string(),
    //   vec![
    //     "/Users/ityuany/GitRepository/wms/src/component/search-queries"
    //       .to_string(),
    //   ],
    // );
    // alias.insert(
    //   "@shein-components/dateRangePicker2".to_string(),
    //   vec![
    //     "/Users/ityuany/GitRepository/wms/src/component/public-component/dateRangePicker2Wrapper"
    //       .to_string(),
    //   ],
    // );
    // alias.insert(
    //   "@".to_string(),
    //   vec!["/Users/ityuany/GitRepository/wms/src".to_string()],
    // );

    let op = Options {
      cwd: Some("/Users/ityuany/GitRepository/bmas/src".to_string()),
      modules: Some(vec!["node_modules".into(), "web_modules".into()]),
      pattern: None,
      ignore: None,
      alias: Some(alias),
    };

    // let result = get_dependents(
    //   "/Users/ityuany/GitRepository/wms/src/lib/dealFunc.js".to_string(),
    //   Some(op.clone()),
    // );
    // if let Ok(result) = result {
    //   for x in result {
    //     println!("{}", x);
    //   }
    // }

    let result1 = check_cycle(Some(op.clone())).unwrap();

    for x in result1 {
      println!("\n跨越{}个文件的循环依赖", x.len());
      for y in x.iter() {
        // println!("{}  {} ", y.from, y.to);
        println!(
          "{}:{}..{}",
          y.source, y.ast_node.loc.start.line, y.ast_node.loc.end.line
        );
      }
    }
  }
}
