use std::sync::{
  atomic::{AtomicU32, Ordering},
  Arc,
};

use anyhow::Context;
use beans::AstNode;
use bimap::BiMap;
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_resolver::{AliasValue, ResolveContext, ResolveOptions, Resolver};
use oxc_span::SourceType;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::{glob_by, read_file_content, GlobArgs};

use crate::model::{Args, Edge, Graphics};

pub async fn get_edges(args: Args) -> anyhow::Result<Graphics> {
  let glob_args = GlobArgs {
    cwd: args.cwd.clone(),
    pattern: args.pattern,
    ignore: args.ignore,
  };

  let resolver_alias: Vec<(String, Vec<AliasValue>)> = args
    .alias
    .clone()
    .into_iter()
    .map(|(key, values)| {
      (key, values.into_iter().map(AliasValue::Path).collect())
    })
    .collect();

  let resolve_options = ResolveOptions {
    alias: resolver_alias,
    modules: args.modules,
    extensions: vec![
      ".ts".into(),
      ".js".into(),
      ".jsx".into(),
      ".tsx".into(),
      ".json".into(),
    ],
    ..ResolveOptions::default()
  };

  let resolver = Resolver::new(resolve_options);

  let shared_context =
    Arc::new(parking_lot::Mutex::new(ResolveContext::default()));

  let responses = tokio::task::spawn_blocking({
    let context = shared_context.clone();
    move || {
      glob_by(
        |path| {
          let allocator = Allocator::default();
          let source_text = read_file_content(path).unwrap();
          let source_type = SourceType::from_path(path).unwrap();
          let ret = Parser::new(&allocator, &source_text, source_type).parse();
          let source = path.display().to_string();
          let source_dir = path.parent();

          let res: Vec<Edge> = ret
            .module_record
            .import_entries
            .par_iter()
            // .iter()
            // .map(|item| &item.module_request)
            .filter_map(|item| {
              let item = &item.module_request;
              let name_str = item.name.to_string();
              // let bi_map_guard = bi_map.blocking_lock();

              // let source_id = match bi_map_guard.get_by_left(&path_str) {
              //   Some(source_id) => source_id.to_string(),
              //   None => {
              //     let id = id_counter.fetch_add(1, Ordering::SeqCst);
              //     let mut bi_map_guard = bi_map.blocking_lock();
              //     bi_map_guard.insert(id.to_string(), path_str.clone());
              //     id.to_string()
              //   }
              // };

              if let Some(source_dir) = source_dir {
                let resolved = resolver.resolve_with_context(
                  &source_dir,
                  &name_str,
                  &mut context.lock(),
                );
                if let Ok(resolved) = resolved {
                  let target =
                    resolved.full_path().to_string_lossy().to_string();

                  return Some(Edge {
                    source: source.clone(),
                    target: target,
                    ast_node: AstNode::with_source_and_span(
                      &source_text,
                      &item.span,
                    ),
                  });
                }
              }

              return None;
            })
            .collect();
          Some(res)
        },
        &glob_args,
      )
    }
  })
  .await
  .with_context(|| "get edges failed")?;

  let id_counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));
  let mut bi_map: BiMap<String, String> = BiMap::new();

  let edges = responses?
    .into_iter()
    .map(|x| x.clone())
    .flatten()
    .map(|x| {
      let source_id = if let Some(source_id) = bi_map.get_by_left(&x.source) {
        source_id.to_string()
      } else {
        let id = id_counter.fetch_add(1, Ordering::SeqCst);
        bi_map.insert(id.to_string(), x.source.clone());
        id.to_string()
      };

      let target_id = if let Some(target_id) = bi_map.get_by_left(&x.target) {
        target_id.to_string()
      } else {
        let id = id_counter.fetch_add(1, Ordering::SeqCst);
        bi_map.insert(id.to_string(), x.target.clone());
        id.to_string()
      };

      Edge {
        source: source_id,
        target: target_id,
        ast_node: x.ast_node.clone(),
      }
    })
    .collect::<Vec<_>>();

  Ok(Graphics {
    dictionaries: bi_map.into_iter().collect(),
    graph: edges,
    invalid_syntax_files: vec![],
  })

  // responses.unwrap().iter().for_each(|x| {
  //   println!("{:?}", x);
  // });
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  use std::time::Instant;

  use super::*;

  #[tokio::test]
  async fn test_get_edges() {
    let start = Instant::now();

    let x = get_edges(Args {
      cwd: "/Users/10015448/Git/gtms".to_string(),
      pattern: "**/*.{js,ts,jsx,tsx}".to_string(),
      ignore: vec![],
      alias: HashMap::new(),
      modules: vec!["node_modules".to_string()],
    })
    .await;

    let duration = start.elapsed();
    println!("执行耗时: {:?}", duration);
  }
}
