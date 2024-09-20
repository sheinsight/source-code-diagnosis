use anyhow::Result;
use log::debug;
use napi_derive::napi;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use petgraph::{
  algo::{is_cyclic_directed, kosaraju_scc},
  graphmap::DiGraphMap,
};
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

pub fn get_node(options: Option<Options>) -> Result<Vec<(String, String)>> {
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

      let mut inline_usages: Vec<(String, String)> = Vec::new();

      SemanticBuilder::file(path.clone())
        .build_handler()
        .each_node(|_handler, node| {
          if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
            let value = import_declaration.source.value.to_string();
            debug!("value: {}", value);

            #[cfg(windows)]
            let normalized_value = value.clone().replace('\\', "/");

            #[cfg(not(windows))]
            let normalized_value = value.clone();

            debug!("normalized_value: {}", normalized_value);

            if let Some(parent) = path.parent() {
              debug!("parent: {}", parent.display().to_string());
              let resolved = resolver.resolve(&parent, &normalized_value);
              if let Ok(resolved_path) = resolved {
                debug!(
                  "resolved_path: {}",
                  resolved_path.full_path().display().to_string()
                );
                inline_usages.push((
                  path.display().to_string(),
                  resolved_path.full_path().display().to_string(),
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

fn normalize_path(path: &str) -> &str {
  #[cfg(windows)]
  {
    path
      .split(':')
      .map_or(path.to_string(), |p| {
        format!("/{}", p.trim_start_matches('\\'))
      })
      .as_str()
  }
  #[cfg(not(windows))]
  {
    path
  }
}

pub fn get_dependents(
  file: String,
  options: Option<Options>,
) -> Result<HashSet<String>> {
  let used = get_node(options)?;
  let mut graph = DiGraphMap::new();
  for (key, value) in used.iter() {
    graph.add_edge(normalize_path(key.as_str()), value.as_str(), ());
  }
  Ok(
    graph
      .neighbors_directed(&file, petgraph::Direction::Incoming)
      .map(|x| x.to_string())
      .collect(),
  )
}

pub fn detect_cycle(options: Option<Options>) -> Result<Vec<Vec<String>>> {
  let used = get_node(options)?;
  let mut graph = DiGraphMap::new();
  for (key, value) in used.iter() {
    graph.add_edge(normalize_path(key.as_str()), value.as_str(), ());
  }
  let files = kosaraju_scc(&graph)
    .into_iter()
    .filter(|scc| {
      scc.len() > 1 || (scc.len() == 1 && graph.contains_edge(scc[0], scc[0]))
    })
    .map(|scc| scc.into_iter().map(|x| x.to_string()).collect())
    .collect();
  Ok(files)
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

    // let op = Options {
    //   cwd: Some("/Users/10015448/Git/bmas/src".to_string()),
    //   modules: Some(vec!["node_modules".into(), "web_modules".into()]),
    //   pattern: None,
    //   ignore: None,
    //   alias: Some(alias),
    // };

    // let result = get_dependents(
    //   "/Users/ityuany/GitRepository/wms/src/lib/dealFunc.js".to_string(),
    //   Some(op.clone()),
    // );
    // if let Ok(result) = result {
    //   for x in result {
    //     println!("{}", x);
    //   }
    // }

    // let result1 = detect_cycle(Some(op.clone())).unwrap();

    // for x in result1 {
    //   println!("\n跨越{}个文件的循环依赖", x.len());
    //   for y in x.iter() {
    //     println!("{}", y);
    //   }
    // }
  }
}