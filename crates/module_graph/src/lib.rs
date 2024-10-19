use anyhow::Result;
use beans::{AstNode, Span};
use bimap::BiMap;
use camino::Utf8PathBuf;
use log::debug;
use napi_derive::napi;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use oxc_span::SourceType;
use petgraph::algo::kosaraju_scc;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::Dfs;
use petgraph::Direction;
use serde::Serialize;
use std::fs::read_to_string;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;
use std::{
  collections::{HashMap, HashSet},
  path::PathBuf,
  sync::{Arc, Mutex},
};
use utils::{glob, GlobOptions, SemanticBuilder};
pub mod graph;

#[napi[object]]
#[derive(Debug, Clone)]
pub struct Options {
  pub cwd: Option<String>,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

pub fn get_node(
  options: Option<Options>,
) -> Result<(Vec<Edge>, BiMap<String, String>)> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let id_counter = Arc::new(AtomicU32::new(0));
  let bi_map: Arc<Mutex<BiMap<String, String>>> =
    Arc::new(Mutex::new(BiMap::new()));

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

  let cwd = match &options {
    Some(Options { cwd: Some(cwd), .. }) => {
      Utf8PathBuf::from(cwd).join("").into_string()
    }
    _ => String::new(),
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
    let bi_map = Arc::clone(&bi_map);

    move |path: PathBuf| {
      debug!("path: {}", path.display().to_string());

      let mut inline_usages: Vec<Edge> = Vec::new();

      let source_code = read_to_string(&path).unwrap();

      let source_type = SourceType::from_path(&path).unwrap();

      SemanticBuilder::code(&source_code, source_type)
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

                let source =
                  path.display().to_string().replace(cwd.as_str(), "");

                let target = Utf8PathBuf::from(
                  resolved_path.full_path().display().to_string(),
                )
                .to_string()
                .replace(cwd.as_str(), "");

                let mut bi_map = bi_map.lock().unwrap();

                let source_id = {
                  if bi_map.contains_left(&source) {
                    bi_map.get_by_left(&source).unwrap().to_string()
                  } else {
                    let id =
                      id_counter.fetch_add(1, Ordering::SeqCst).to_string();
                    bi_map.insert(source.clone(), id.to_string());
                    id.to_string()
                  }
                };

                let target_id = {
                  if bi_map.contains_left(&target) {
                    bi_map.get_by_left(&target).unwrap().to_string()
                  } else {
                    let id =
                      id_counter.fetch_add(1, Ordering::SeqCst).to_string();
                    bi_map.insert(target.clone(), id.to_string());
                    id.to_string()
                  }
                };

                inline_usages.push(Edge {
                  source: source_id,
                  target: target_id,
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
  let y = bi_map.lock().unwrap().clone();

  Ok((x, y))
}

pub fn build_graph(
  used: &[Edge],
) -> (
  DiGraph<String, ()>,
  HashMap<(String, String), &Edge>,
  HashMap<String, NodeIndex>,
) {
  let len = used.len();

  let mut graph = DiGraph::with_capacity(len, len);
  let mut module_map = HashMap::with_capacity(len);
  let mut node_indices = HashMap::with_capacity(len);

  for edge in used.iter() {
    let source_node = *node_indices
      .entry(edge.source.clone())
      .or_insert_with(|| graph.add_node(edge.source.clone()));
    let target_node = *node_indices
      .entry(edge.target.clone())
      .or_insert_with(|| graph.add_node(edge.target.clone()));
    graph.add_edge(source_node, target_node, ());
    module_map.insert((edge.source.clone(), edge.target.clone()), edge);
  }

  (graph, module_map, node_indices)
}

pub fn get_dependents(
  file: String,
  options: Option<Options>,
) -> Result<Graphics> {
  let (used, bimap) = get_node(options)?;

  let (graph, module_map, node_indices) = build_graph(&used);

  let file_id = bimap.get_by_left(&file).unwrap();
  let target_index = *node_indices.get(file_id).unwrap();

  let mut result = Vec::new();
  let mut visited = HashSet::new();
  let mut dictionaries = HashMap::new();

  fn traverse_neighbors(
    current: NodeIndex,
    dictionaries: &mut HashMap<String, String>,
    graph: &DiGraph<String, ()>,
    module_map: &HashMap<(String, String), &Edge>,
    bimap: &BiMap<String, String>,
    visited: &mut HashSet<NodeIndex>,
    result: &mut Vec<Edge>,
  ) {
    if visited.contains(&current) {
      return;
    }
    visited.insert(current);

    for neighbor in graph.neighbors_directed(current, Direction::Incoming) {
      let source = graph[neighbor].to_string();
      let target = graph[current].to_string();
      let source_file_path = bimap.get_by_right(&source).unwrap();
      let target_file_path = bimap.get_by_right(&target).unwrap();

      dictionaries.insert(source.to_string(), source_file_path.to_string());
      dictionaries.insert(target.to_string(), target_file_path.to_string());

      result.push(Edge {
        source: source.to_string(),
        target: target.to_string(),
        ast_node: module_map[&(source, target)].ast_node.clone(),
      });

      traverse_neighbors(
        neighbor,
        dictionaries,
        graph,
        module_map,
        bimap,
        visited,
        result,
      );
    }
  }

  traverse_neighbors(
    target_index,
    &mut dictionaries,
    &graph,
    &module_map,
    &bimap,
    &mut visited,
    &mut result,
  );

  Ok(Graphics {
    dictionaries: dictionaries,
    // dictionaries: bimap.clone().into_iter().map(|(l, r)| (r, l)).collect(),
    graph: result,
  })
}

pub fn get_dependencies(
  file: String,
  options: Option<Options>,
) -> Result<Graphics> {
  let (used, bimap) = get_node(options)?;
  let (graph, module_map, node_indices) = build_graph(&used);

  let file_id = bimap.get_by_left(&file).unwrap();
  let target_index = *node_indices.get(file_id).unwrap();

  let mut result = Vec::new();
  let mut visited = HashSet::new();
  let mut dictionaries = HashMap::new();

  fn traverse_neighbors(
    current: NodeIndex,
    dictionaries: &mut HashMap<String, String>,
    graph: &DiGraph<String, ()>,
    module_map: &HashMap<(String, String), &Edge>,
    bimap: &BiMap<String, String>,
    visited: &mut HashSet<NodeIndex>,
    result: &mut Vec<Edge>,
  ) {
    if visited.contains(&current) {
      return;
    }
    visited.insert(current);

    for neighbor in graph.neighbors_directed(current, Direction::Outgoing) {
      let source = graph[current].to_string();
      let target = graph[neighbor].to_string();
      let source_file_path = bimap.get_by_right(&source).unwrap();
      let target_file_path = bimap.get_by_right(&target).unwrap();

      dictionaries.insert(source.to_string(), source_file_path.to_string());
      dictionaries.insert(target.to_string(), target_file_path.to_string());

      result.push(Edge {
        source: source.to_string(),
        target: target.to_string(),
        ast_node: module_map[&(source, target)].ast_node.clone(),
      });

      traverse_neighbors(
        neighbor,
        dictionaries,
        graph,
        module_map,
        bimap,
        visited,
        result,
      );
    }
  }

  traverse_neighbors(
    target_index,
    &mut dictionaries,
    &graph,
    &module_map,
    &bimap,
    &mut visited,
    &mut result,
  );

  Ok(Graphics {
    dictionaries: dictionaries,
    // dictionaries: bimap.clone().into_iter().map(|(l, r)| (r, l)).collect(),
    graph: result,
  })
}

#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
pub struct Edge {
  pub source: String,
  pub target: String,
  pub ast_node: AstNode,
}

#[napi(object)]
pub struct GroupGraphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Vec<Edge>>,
}

#[napi(object)]
pub struct Graphics {
  pub dictionaries: HashMap<String, String>,
  pub graph: Vec<Edge>,
}

pub fn check_cycle(options: Option<Options>) -> Result<GroupGraphics> {
  let (used, bimap) = get_node(options)?;

  let (graph, module_map, node_indices) = build_graph(&used);

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
                let source = graph[window[0]].clone();
                let target = graph[window[1]].clone();
                Edge {
                  source: source.to_string(),
                  target: target.to_string(),
                  ast_node: module_map[&(source, target)].ast_node.clone(),
                }
              })
              .collect::<Vec<Edge>>();

            let last = stack.last().unwrap();
            cycle.push(Edge {
              source: graph[*last].clone(),
              target: graph[neighbor].clone(),
              ast_node: module_map
                [&(graph[*last].clone(), graph[neighbor].clone())]
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

  Ok(GroupGraphics {
    dictionaries: bimap.clone().into_iter().map(|(l, r)| (r, l)).collect(),
    graph: result,
  })
}

fn normalize_cycle(mut cycle: Vec<Edge>) -> Vec<Edge> {
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
