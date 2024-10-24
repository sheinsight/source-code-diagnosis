use std::{
  collections::{HashMap, HashSet},
  fs::read_to_string,
  path::PathBuf,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex,
  },
};

use anyhow::Result;
use beans::{AstNode, Location, Span};
use bimap::BiMap;
use camino::Utf8PathBuf;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use oxc_span::SourceType;
use petgraph::{
  algo::kosaraju_scc,
  graph::{DiGraph, NodeIndex},
  visit::Dfs,
  Direction,
};
use rayon::prelude::*;
use utils::SemanticBuilder;
use wax::{Glob, WalkEntry, WalkError};

use crate::model::{Args, Edge, Graphics, GroupGraphics};

pub struct Graph<'a> {
  id_counter: Arc<AtomicU32>,
  bi_map: Arc<Mutex<BiMap<String, String>>>,
  pub edges: Arc<Mutex<Vec<Edge>>>,
  node_index_map: HashMap<String, NodeIndex>,
  edge_map: HashMap<(String, String), Edge>,
  cwd: String,
  resolver: Resolver,
  entries: Vec<Result<WalkEntry<'a>, WalkError>>,
}

impl<'a> Graph<'a> {
  pub fn new(options: Args) -> Self {
    let resolver =
      Self::build_resolver(options.alias.clone(), options.modules.clone());
    let entries =
      Self::build_entries(&options.cwd, &options.pattern, options.ignore);
    Self {
      id_counter: Arc::new(AtomicU32::new(0)),
      bi_map: Arc::new(Mutex::new(BiMap::new())),
      edges: Arc::new(Mutex::new(Vec::new())),
      node_index_map: HashMap::new(),
      edge_map: HashMap::new(),
      cwd: options.cwd,
      resolver,
      entries,
    }
  }

  fn build_resolver(
    alias: HashMap<String, Vec<String>>,
    modules: Vec<String>,
  ) -> Resolver {
    let resolver_alias: Vec<(String, Vec<AliasValue>)> = alias
      .clone()
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
    resolver
  }

  fn build_entries(
    cwd: &str,
    pattern: &str,
    ignore: Vec<String>,
  ) -> Vec<Result<WalkEntry<'a>, WalkError>> {
    if let Ok(glob) = Glob::new(pattern) {
      if let Ok(entries) =
        glob.walk(&cwd).not(ignore.iter().map(|s| s.as_str()))
      {
        return entries.collect::<Vec<_>>();
      }
    }
    Vec::new()
  }
}

const END_ID: &str = "__END__";

impl<'a> Graph<'a> {
  fn build_edges(&mut self) {
    let empty_id = self.build_id(END_ID);
    self.entries.par_iter().for_each(|item| {
      let entry = match item {
        Ok(entry) => entry,
        Err(_) => return,
      };

      let path = entry.path();
      if !path.is_file() {
        return;
      }

      if path.components().any(|c| c.as_os_str() == "node_modules") {
        return;
      }

      let source_code = match read_to_string(&path) {
        Ok(code) => code,
        Err(_) => return,
      };

      let source_type = match SourceType::from_path(&path) {
        Ok(st) => st,
        Err(_) => return,
      };

      let builder = SemanticBuilder::code(&source_code, source_type);
      let handler = builder.build_handler();
      let nodes = handler.semantic.nodes();

      let mut thread_edges: Vec<Edge> = Vec::new();

      for node in nodes.iter() {
        let source = self.to_relative_path(&self.cwd, path.to_path_buf());
        let source_id = self.build_id(&source);
        let import_declaration = match node.kind() {
          AstKind::ImportDeclaration(id) => id,
          _ => {
            let span = Span { start: 0, end: 0 };
            let loc = Location {
              start: beans::Position { line: 0, col: 0 },
              end: beans::Position { line: 0, col: 0 },
            };
            let edge = Edge {
              source: source_id,
              target: empty_id.clone(),
              ast_node: AstNode { span, loc },
            };
            thread_edges.push(edge);
            continue;
          }
        };

        let value = import_declaration.source.value.to_string();
        let parent = match path.parent() {
          Some(p) => p,
          None => continue,
        };

        let resolved_path = match self.resolver.resolve(&parent, &value) {
          Ok(rp) => rp,
          Err(_) => continue,
        };

        if resolved_path
          .full_path()
          .components()
          .any(|c| c.as_os_str() == "node_modules")
        {
          continue;
        }

        let (span, loc) = handler.get_node_box(node);

        let target = self
          .to_relative_path(&self.cwd, resolved_path.full_path().to_path_buf());
        let target_id = self.build_id(&target);

        let edge = self.build_edge(source_id, target_id, span, loc);
        thread_edges.push(edge);
      }

      if let Ok(mut edges) = self.edges.lock() {
        edges.extend(thread_edges);
      }
    });
  }

  pub fn check_cycle(&mut self) -> Result<GroupGraphics> {
    self.build_edges();

    let graph = self.build_graph()?;

    let mut result = Vec::new();

    let sccs = kosaraju_scc(&graph);

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
                  if let Some(edge) =
                    self.edge_map.get(&(source.clone(), target.clone()))
                  {
                    Edge {
                      source: source.clone(),
                      target: target.clone(),
                      ast_node: edge.ast_node.clone(),
                    }
                  } else {
                    Edge {
                      source: source.clone(),
                      target: target.clone(),
                      ast_node: AstNode::default(),
                    }
                  }
                })
                .collect::<Vec<Edge>>();

              if let Some(node) = stack.last() {
                cycle.push(Edge {
                  source: graph[*node].clone(),
                  target: graph[neighbor].clone(),
                  ast_node: self.edge_map
                    [&(graph[*node].clone(), graph[neighbor].clone())]
                    .ast_node
                    .clone(),
                });
              }

              // 标准化循环并添加到HashSet中
              let normalized_cycle = self.normalize_cycle(cycle);
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
      dictionaries: self.get_dictionaries(),
      graph: result,
    })
  }

  pub fn check_dependents(&mut self, file: String) -> Result<Graphics> {
    let graphics = self.deep_neighbors_directed(file, Direction::Incoming)?;
    Ok(graphics)
  }

  pub fn check_dependencies(&mut self, file: String) -> Result<Graphics> {
    let graphics = self.deep_neighbors_directed(file, Direction::Outgoing)?;
    Ok(graphics)
  }

  fn deep_neighbors_directed(
    &mut self,
    file: String,
    direction: Direction,
  ) -> Result<Graphics> {
    self.build_edges();
    let graph = self.build_graph()?;

    let target_index = {
      let bin_map = self
        .bi_map
        .lock()
        .map_err(|_| anyhow::anyhow!("bi_map lock failed"))?;

      let file_id = bin_map
        .get_by_left(&file)
        .ok_or(anyhow::anyhow!("file not found"))?;

      let target_index = self
        .node_index_map
        .get(file_id)
        .ok_or(anyhow::anyhow!("file not found"))?;

      *target_index
    };

    let mut result = Vec::new();
    let mut visited = HashSet::new();
    let mut dictionaries = HashMap::new();

    self.traverse_neighbors(
      target_index,
      &mut dictionaries,
      &graph,
      &mut visited,
      &mut result,
      direction,
    );

    Ok(Graphics {
      dictionaries,
      graph: result,
    })
  }

  fn traverse_neighbors(
    &mut self,
    current: NodeIndex,
    dictionaries: &mut HashMap<String, String>,
    graph: &DiGraph<String, ()>,
    visited: &mut HashSet<NodeIndex>,
    result: &mut Vec<Edge>,
    direction: Direction,
  ) {
    if visited.contains(&current) {
      return;
    }
    visited.insert(current);

    for neighbor in graph.neighbors_directed(current, direction) {
      let (source, target) = match direction {
        Direction::Incoming => {
          let source = graph[neighbor].to_string();
          let target = graph[current].to_string();
          (source, target)
        }
        Direction::Outgoing => {
          let source = graph[current].to_string();
          let target = graph[neighbor].to_string();
          (source, target)
        }
      };

      if let Ok(bin_map) = self.bi_map.lock() {
        let source_file_path = bin_map.get_by_right(&source).unwrap();
        let target_file_path = bin_map.get_by_right(&target).unwrap();

        if END_ID == source_file_path || END_ID == target_file_path {
          continue;
        }

        dictionaries.insert(source.to_string(), source_file_path.to_string());
        dictionaries.insert(target.to_string(), target_file_path.to_string());
      }

      let edge = self
        .edge_map
        .get(&(source.clone(), target.clone()))
        .unwrap();

      result.push(Edge {
        source: source.clone(),
        target: target.clone(),
        ast_node: edge.ast_node.clone(),
      });

      self.traverse_neighbors(
        neighbor,
        dictionaries,
        graph,
        visited,
        result,
        direction,
      );
    }
  }

  fn get_dictionaries(&self) -> HashMap<String, String> {
    if let Ok(bin_map) = self.bi_map.lock() {
      bin_map.clone().into_iter().map(|(l, r)| (r, l)).collect()
    } else {
      HashMap::new()
    }
  }

  fn normalize_cycle(&self, mut cycle: Vec<Edge>) -> Vec<Edge> {
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

  fn build_graph(&mut self) -> Result<DiGraph<String, ()>> {
    if let Ok(edges) = self.edges.lock() {
      let len = edges.len();
      let mut graph = DiGraph::<String, ()>::with_capacity(len, len);
      for edge in edges.iter() {
        let source_node = *self
          .node_index_map
          .entry(edge.source.clone())
          .or_insert_with_key(|key| graph.add_node(key.clone()));

        let target_node = *self
          .node_index_map
          .entry(edge.target.clone())
          .or_insert_with_key(|key| graph.add_node(key.clone()));

        graph.add_edge(source_node, target_node, ());

        self
          .edge_map
          .insert((edge.source.clone(), edge.target.clone()), edge.clone());
      }

      Ok(graph)
    } else {
      Err(anyhow::anyhow!("edges lock failed"))
    }
  }

  fn build_id(&self, node: &str) -> String {
    if let Ok(mut bin_map) = self.bi_map.lock() {
      if let Some(id) = bin_map.get_by_left(node) {
        id.to_string()
      } else {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        bin_map.insert(node.to_string(), id.to_string());
        id.to_string()
      }
    } else {
      eprintln!("bi_map lock failed");
      "".to_string()
    }
  }

  fn to_relative_path(
    &self,
    cwd: &String,
    absolute_path_buf: PathBuf,
  ) -> String {
    if let Ok(absolute_path) = Utf8PathBuf::from_path_buf(absolute_path_buf) {
      absolute_path.to_string().replace(cwd, "")
    } else {
      "".to_string()
    }
  }

  fn build_edge(
    &self,
    source_id: String,
    target_id: String,
    span: oxc_span::Span,
    loc: Location,
  ) -> Edge {
    Edge {
      source: source_id,
      target: target_id,
      ast_node: AstNode {
        span: Span {
          start: span.start,
          end: span.end,
        },
        loc,
      },
    }
  }
}
