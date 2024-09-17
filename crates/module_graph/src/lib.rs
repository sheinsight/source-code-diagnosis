use anyhow::Result;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use petgraph::{algo::is_cyclic_directed, graphmap::DiGraphMap};
use std::{
  collections::{HashMap, HashSet},
  path::PathBuf,
  sync::{Arc, Mutex},
};
use utils::{glob, GlobOptions, SemanticBuilder};

#[derive(Debug, Clone)]
pub struct Options {
  pub cwd: Option<String>,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
}

pub fn get_node(options: Option<Options>) -> Result<Vec<(String, String)>> {
  let used = Arc::new(Mutex::new(Vec::new()));

  let alias = options
    .as_ref()
    .and_then(|opt| opt.alias.as_ref())
    .cloned()
    .unwrap_or_default();
  let resolver_alias: Vec<(String, Vec<AliasValue>)> = alias
    .into_iter()
    .map(|(key, values)| {
      (
        key,
        values
          .into_iter()
          .map(|v| AliasValue::Path(v.into()))
          .collect(),
      )
    })
    .collect();
  let resolver = Resolver::new(ResolveOptions {
    alias: resolver_alias,
    extensions: vec![".ts".into(), ".js".into(), ".jsx".into(), ".tsx".into()],
    ..ResolveOptions::default()
  });

  let handler = {
    let used = Arc::clone(&used);

    move |path: PathBuf| {
      let mut inline_usages: Vec<(String, String)> = Vec::new();

      SemanticBuilder::file(path.clone())
        .build_handler()
        .each_node(|handler, semantic, node| {
          if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
            let value = import_declaration.source.value.to_string();
            let is_relative_path =
              value.starts_with("./") || value.starts_with("../");
            if is_relative_path {
              let resolved_path = resolver
                .resolve(&path.parent().unwrap(), &value.clone())
                .unwrap();
              inline_usages.push((
                path.display().to_string(),
                resolved_path.full_path().display().to_string(),
              ));
            } else {
              // println!("非相对路径: {}, 当前文件: {}", value, path.display());
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
) -> Result<HashSet<String>> {
  let used = get_node(options)?;
  let mut graph = DiGraphMap::new();
  for (key, value) in used.iter() {
    graph.add_edge(key.as_str(), value.as_str(), ());
  }
  Ok(
    graph
      .neighbors_directed(&file, petgraph::Direction::Incoming)
      .map(|x| x.to_string())
      .collect(),
  )
}

pub fn detect_cycle(options: Option<Options>) -> Result<bool> {
  let used = get_node(options)?;
  let mut graph = DiGraphMap::new();
  for (key, value) in used.iter() {
    graph.add_edge(key.as_str(), value.as_str(), ());
  }
  Ok(is_cyclic_directed(&graph))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_check_danger_strings() {
    let mut alias = HashMap::new();
    alias.insert(
      "@src".to_string(),
      vec!["/Users/ityuany/GitRepository/wms/src".to_string()],
    );
    alias.insert(
      "@public-component".to_string(),
      vec![
        "/Users/ityuany/GitRepository/wms/src/component/public-component"
          .to_string(),
      ],
    );
    alias.insert(
      "@search-queries".to_string(),
      vec![
        "/Users/ityuany/GitRepository/wms/src/component/search-queries"
          .to_string(),
      ],
    );
    alias.insert(
      "@shein-components/dateRangePicker2".to_string(),
      vec![
        "/Users/ityuany/GitRepository/wms/src/component/public-component/dateRangePicker2Wrapper"
          .to_string(),
      ],
    );
    alias.insert(
      "@".to_string(),
      vec!["/Users/ityuany/GitRepository/wms/src".to_string()],
    );

    let op = Options {
      cwd: Some("/Users/ityuany/GitRepository/wms/src".to_string()),
      pattern: None,
      ignore: None,
      alias: Some(alias),
    };

    let result = get_dependents(
      "/Users/ityuany/GitRepository/wms/src/lib/dealFunc.js".to_string(),
      Some(op.clone()),
    );
    if let Ok(result) = result {
      for x in result {
        println!("{}", x);
      }
    }

    let result1 = detect_cycle(Some(op.clone()));
    println!("是否存在循环依赖: {:?}", result1);
  }
}
