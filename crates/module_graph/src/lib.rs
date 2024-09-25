use anyhow::Result;
use beans::{AstNode, Span};
use log::debug;
use napi_derive::napi;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use petgraph::{
  algo::{is_cyclic_directed, kosaraju_scc},
  graph::{DiGraph, NodeIndex},
  graphmap::DiGraphMap,
  visit::Dfs,
};
use std::{
  collections::{HashMap, HashSet, VecDeque},
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

pub fn get_node(options: Option<Options>) -> Result<Vec<(String, Dependency)>> {
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

      let mut inline_usages: Vec<(String, Dependency)> = Vec::new();

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

                inline_usages.push((
                  path.display().to_string(),
                  Dependency {
                    from: path.display().to_string(),
                    to: resolved_path.full_path().display().to_string(),
                    ast_node: AstNode {
                      span: Span {
                        start: span.start,
                        end: span.end,
                      },
                      loc: loc,
                    },
                  },
                ));
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

pub fn get_dependents(
  file: String,
  options: Option<Options>,
) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;

  let mut module_map = HashMap::new();

  let mut graph = DiGraph::new();
  let mut node_indices = HashMap::new();

  for (_, value) in used.iter() {
    let from = value.from.as_str();
    let to = value.to.as_str();
    let from_node = *node_indices
      .entry(from)
      .or_insert_with(|| graph.add_node(from));
    let to_node = *node_indices.entry(to).or_insert_with(|| graph.add_node(to));

    module_map.insert((from, to), value);

    graph.add_edge(from_node, to_node, ());
  }

  let mut dependency_paths: Vec<Vec<Cycle>> = Vec::new();

  if let Some(&start_index) = node_indices.get(file.as_str()) {
    let mut queue = VecDeque::new();

    queue.push_back(vec![start_index]);

    while let Some(path) = queue.pop_front() {
      let current = *path.last().unwrap();

      let is_end = graph
        .neighbors_directed(current, petgraph::Direction::Incoming)
        .next()
        .is_none();

      if is_end {
        let mut inline_result = Vec::new();

        println!(
          "path: {:?}",
          path.iter().map(|x| graph[*x]).collect::<Vec<&str>>()
        );

        for (index, node) in path.iter().enumerate() {
          let from = graph[*node].to_string();
          let to = if index == path.len() - 1 {
            graph[start_index].to_string()
          } else {
            graph[path[index + 1]].to_string()
          };
          if let Some(dependency) =
            module_map.get(&(to.as_str(), from.as_str()))
          {
            inline_result.push(Cycle {
              from: from.clone(),
              to: to.clone(),
              ast_node: dependency.ast_node,
            });
          }
        }
        if !inline_result.is_empty() {
          dependency_paths.push(inline_result);
        }
      }

      for neighbor in
        graph.neighbors_directed(current, petgraph::Direction::Incoming)
      {
        let mut new_path = path.clone();
        new_path.push(neighbor);
        queue.push_back(new_path);
      }
    }
  } else {
    println!("File {} not found in the dependency graph", file);
  }

  Ok(dependency_paths)
}

pub fn get_dependencies(
  file: String,
  options: Option<Options>,
) -> Result<HashSet<String>> {
  let used = get_node(options)?;
  let mut graph = DiGraphMap::new();
  for (key, value) in used.iter() {
    let from = key.as_str();
    let to = value.to.as_str();
    graph.add_edge(from, to, ());
  }
  Ok(
    graph
      .neighbors_directed(&file, petgraph::Direction::Outgoing)
      .map(|x| x.to_string())
      .collect(),
  )
}

#[napi(object)]
#[derive(Debug)]
pub struct Cycle {
  pub from: String,
  pub ast_node: AstNode,
  pub to: String,
}

fn dfs_cycle<'a>(
  graph: &'a DiGraph<&'a str, ()>,
  node: petgraph::prelude::NodeIndex,
  visited: &mut HashSet<petgraph::prelude::NodeIndex>,
  path: &mut Vec<&'a str>,
  cycles: &mut Vec<Vec<&'a str>>,
) {
  visited.insert(node);
  path.push(graph[node]);

  for neighbor in graph.neighbors(node) {
    if !visited.contains(&neighbor) {
      dfs_cycle(graph, neighbor, visited, path, cycles);
    } else if path.contains(&graph[neighbor]) {
      let cycle_start =
        path.iter().position(|&x| x == graph[neighbor]).unwrap();
      cycles.push(path[cycle_start..].to_vec());
    }
  }

  path.pop();
}

pub fn detect_cycle(options: Option<Options>) -> Result<Vec<Vec<Cycle>>> {
  let used = get_node(options)?;

  let mut module_map = HashMap::new();

  let mut graph = DiGraph::new();
  let mut node_map: HashMap<&str, NodeIndex> = HashMap::new();

  for (_, value) in used.iter() {
    let from = value.from.as_str();
    let to = value.to.as_str();

    let from_node =
      *node_map.entry(from).or_insert_with(|| graph.add_node(from));

    let to_node = *node_map.entry(to).or_insert_with(|| graph.add_node(to));

    module_map.insert((from, to), value);

    graph.add_edge(from_node, to_node, ());
  }

  let mut cycles = Vec::new();
  let mut visited = HashSet::new();
  let mut path = Vec::new();

  for node in graph.node_indices() {
    if !visited.contains(&node) {
      dfs_cycle(&graph, node, &mut visited, &mut path, &mut cycles);
    }
  }

  let mut result = Vec::new();

  for cycle in cycles.into_iter() {
    let mut inline_result = Vec::new();

    for (index, item) in cycle.iter().enumerate() {
      let from = item.to_string();
      let to = if index == cycle.len() - 1 {
        cycle[0].to_string()
      } else {
        cycle[index + 1].to_string()
      };
      if let Some(dependency) = module_map.get(&(from.as_str(), to.as_str())) {
        inline_result.push(Cycle {
          from: from.clone(),
          to: to.clone(),
          ast_node: dependency.ast_node,
        });
      } else {
        eprintln!("no dependency for {} -> {}", from, to);
      }
    }
    result.push(inline_result);
  }

  Ok(result)
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
          "/Users/10015448/Git/bmas",
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
      cwd: Some("/Users/10015448/Git/bmas/src".to_string()),
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

    let result1 = detect_cycle(Some(op.clone())).unwrap();

    for x in result1 {
      println!("\n跨越{}个文件的循环依赖", x.len());
      for y in x.iter() {
        // println!("{}  {} ", y.from, y.to);
        println!(
          "{}:{}..{}",
          y.from, y.ast_node.loc.start.line, y.ast_node.loc.end.line
        );
      }
    }
  }
}
