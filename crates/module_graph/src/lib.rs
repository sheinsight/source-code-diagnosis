use anyhow::Result;
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
}

pub fn get_node(options: Option<Options>) -> Result<Vec<(String, String)>> {
  let used = Arc::new(Mutex::new(Vec::new()));

  let alias = match &options {
    Some(Options {
      alias: Some(alias), ..
    }) => alias.clone(),
    _ => HashMap::new(),
  };

  let resolver_alias = alias
    .into_iter()
    .map(|(key, values)| {
      (key, values.into_iter().map(AliasValue::Path).collect())
    })
    .collect();

  let resolver = Resolver::new(ResolveOptions {
    alias: resolver_alias,
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
      let mut inline_usages: Vec<(String, String)> = Vec::new();

      SemanticBuilder::file(path.clone())
        .build_handler()
        .each_node(|_handler, node| {
          if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
            let value = import_declaration.source.value.to_string();
            let is_relative_path =
              value.starts_with("./") || value.starts_with("../");
            if is_relative_path {
              if let Ok(resolved_path) =
                resolver.resolve(&path.parent().unwrap(), &value.clone())
              {
                inline_usages.push((
                  path.display().to_string(),
                  resolved_path.full_path().display().to_string(),
                ));
              } else {
                println!(
                  "无法解析引用地址: {},{}",
                  value,
                  path.display().to_string()
                );
              }
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

pub fn detect_cycle(options: Option<Options>) -> Result<Vec<Vec<String>>> {
  let used = get_node(options)?;
  let mut graph = DiGraphMap::new();
  for (key, value) in used.iter() {
    graph.add_edge(key.as_str(), value.as_str(), ());
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
      ("@public-component", "src/public-component"),
      ("@apis", "src/apis"),
      ("@hooks", "src/public-component/hooks"),
      ("@common", "web_modules/common"),
      ("@hoc", "src/component/hoc"),
      ("@shein-lib", "web_modules/shein-lib"),
      ("vscode", "monaco-languageclient/lib/vscode-compatibility"),
    ];

    for (key, value) in x.iter() {
      alias.insert(
        key.to_string(),
        vec![format!(
          "{}/{}",
          "/Users/10015448/Git/pdc",
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
      cwd: Some("/Users/10015448/Git/pdc/src".to_string()),
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
        println!("{}", y);
      }
    }
  }
}
