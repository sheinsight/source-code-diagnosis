use std::{
  path::Path,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use beans::AstNode;
use bimap::BiMap;
use oxc_resolver::{AliasValue, ResolveContext, ResolveOptions, Resolver};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::{glob_by_semantic, GlobArgs, GlobErrorHandler, GlobSuccessHandler};

use crate::model::{Args, Edge, Graphics, TargetMetadata};

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleRecord {
  name: String,
  span: oxc::span::Span,
}

pub fn get_graph(args: Args) -> anyhow::Result<Graphics> {
  let cwd_path = Path::new(&args.cwd);

  let glob_args = GlobArgs {
    cwd: args.cwd.clone(),
    pattern: args.pattern,
    ignore: args.ignore,
  };

  let resolver_alias: Vec<(String, Vec<AliasValue>)> = args
    .resolve
    .alias
    .into_iter()
    .map(|(key, values)| {
      (key, values.into_iter().map(AliasValue::Path).collect())
    })
    .collect();

  let resolve_options = ResolveOptions {
    alias: resolver_alias,
    modules: args.resolve.modules,
    main_files: vec!["index".into()],
    builtin_modules: true,
    fully_specified: false,
    condition_names: vec![
      "import".into(),  // 支持 ESM imports
      "require".into(), // 支持 CommonJS require
      "default".into(), // 默认导出
      "types".into(),   // TypeScript 类型
    ],
    symlinks: false,
    main_fields: vec!["module".into(), "main".into()],
    extensions: args.resolve.extensions,
    ..ResolveOptions::default()
  };

  let resolver = Resolver::new(resolve_options);

  let shared_context =
    Arc::new(parking_lot::Mutex::new(ResolveContext::default()));

  let syntax_errors = Arc::new(parking_lot::Mutex::new(Vec::new()));

  let responses = glob_by_semantic(
    |GlobSuccessHandler {
       parse,
       dir,
       relative_path,
       semantic,
       ..
     }| {
      let source_text = semantic.source_text();

      let module_record = parse.module_record;

      let mr = module_record
        .import_entries
        .iter()
        .map(|item| item.module_request.clone())
        .chain(
          module_record
            .star_export_entries
            .iter()
            .filter_map(|item| item.module_request.clone()),
        )
        .chain(
          module_record
            .indirect_export_entries
            .iter()
            .filter_map(|item| item.module_request.clone()),
        )
        .map(|record| ModuleRecord {
          name: record.name.to_string(),
          span: record.span,
        })
        .collect::<Vec<_>>();

      let res: Vec<Edge> = mr
        .par_iter()
        .filter_map(|item| {
          let name_str = item.name.to_string();

          let resolved = resolver.resolve_with_context(
            &dir,
            &name_str,
            &mut shared_context.lock(),
          );

          if let Ok(resolved) = resolved {
            let target_path =
              pathdiff::diff_paths(resolved.full_path(), cwd_path)?;

            let target = utils::win_path_to_unix(
              &target_path.to_string_lossy().to_string(),
            );

            let target_metadata = may_be_node_modules(&target);

            return Some(Edge {
              source_id: relative_path.clone(),
              target_id: target,
              missing: false,
              target_metadata,
              ast_node: AstNode::with_source_and_span(&source_text, &item.span),
            });
          } else {
            let target_metadata = may_be_node_modules(&name_str);

            return Some(Edge {
              source_id: relative_path.clone(),
              target_id: name_str,
              missing: true,
              target_metadata,
              ast_node: AstNode::with_source_and_span(&source_text, &item.span),
            });
          }
        })
        .collect();
      Some(res)
    },
    |GlobErrorHandler { .. }| {
      return None;
    },
    &glob_args,
  )?
  .into_iter()
  .flatten()
  .collect::<Vec<_>>();

  let id_counter: Arc<AtomicU32> = Arc::new(AtomicU32::new(0));

  let bi_map =
    Arc::new(parking_lot::Mutex::new(BiMap::<String, String>::new()));

  let edges = responses
    .par_iter()
    .filter_map(|x| {
      // 提取获取或创建 ID 的逻辑为一个闭包
      let mut bi_map = bi_map.lock();

      let mut get_or_create_id = |path: &str| -> String {
        bi_map
          .get_by_right(path)
          .map(String::from)
          .unwrap_or_else(|| {
            let id = id_counter.fetch_add(1, Ordering::SeqCst).to_string();
            bi_map.insert(id.clone(), path.to_string());
            id
          })
      };

      let source_id = get_or_create_id(&x.source_id);
      let target_id = get_or_create_id(&x.target_id);

      let mut target_metadata = may_be_node_modules(&x.target_id);
      target_metadata.module_id = get_or_create_id(&target_metadata.module_id);

      Some(Edge {
        source_id,
        target_id,
        target_metadata,
        ..x.clone()
      })
    })
    .collect::<Vec<_>>();

  let syntax_errors = syntax_errors.lock().to_vec();

  let bi_map = bi_map.lock();

  Ok(Graphics {
    dictionaries: bi_map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
    graph: edges,
    syntax_errors: syntax_errors,
  })
}

fn may_be_node_modules(target: &str) -> TargetMetadata {
  const LOCAL_PATTERNS: [&str; 4] = ["./", "../", "/", "@/"];

  let module_name = get_main_module_name(target);

  if target.contains("node_modules") {
    return TargetMetadata {
      module_id: module_name,
      may_be: true,
    };
  }

  if target.starts_with('@') && !target.starts_with("@/") {
    return TargetMetadata {
      module_id: module_name,
      may_be: true,
    };
  }

  let may_be = !LOCAL_PATTERNS
    .iter()
    .any(|pattern| target.contains(pattern));

  if may_be {
    return TargetMetadata {
      module_id: module_name,
      may_be: true,
    };
  }

  return TargetMetadata {
    module_id: module_name,
    may_be: false,
  };
}

fn get_main_module_name(module_name: &str) -> String {
  // e.g. node_modules/antd/lib/Button
  let module_name = module_name.trim_start_matches("node_modules/");

  if module_name.starts_with('@') && !module_name.starts_with("@/") {
    // e.g. @babel/core/lib/something
    module_name.split('/').take(2).collect::<Vec<_>>().join("/")
  } else {
    // e.g. lodash/cloneDeep
    module_name
      .split('/')
      .next()
      .unwrap_or(module_name)
      .to_string()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[macro_export]
  macro_rules! assert_may_be_node_modules {
    {
        #[$test_attr:tt]
        $fn_name:ident,
        $target:expr => $expected:expr
    } => {
        #[$test_attr]
        fn $fn_name() {
            let resp = may_be_node_modules($target);
            assert_eq!(
                resp.may_be, $expected,
                "For target '{}': expected {}, got {}",
                $target, $expected, resp.may_be
            );
        }
    };
  }

  #[macro_export]
  macro_rules! assert_main_module_name {
    {
        #[$test_attr:tt]
        $fn_name:ident,
        $target:expr => $expected:expr
    } => {
        #[$test_attr]
        fn $fn_name() {
            assert_eq!(get_main_module_name($target), $expected);
        }
    };
  }

  assert_main_module_name! {
    #[test]
    test_get_main_module_name_with_node_modules_prefix,
    "node_modules/antd/lib/Button" => "antd"
  }

  assert_main_module_name! {
    #[test]
    test_get_main_module_name_with_scope_prefix,
    "@babel/core/lib/something" => "@babel/core"
  }

  assert_main_module_name! {
    #[test]
    test_get_main_module_name_with_at_prefix_and_slash,
    "lodash/cloneDeep" => "lodash"
  }

  assert_may_be_node_modules! {
    #[test]
    test_may_be_node_modules_with_node_modules_prefix,
    "node_modules/antd/lib/Button" => true
  }

  assert_may_be_node_modules! {
    #[test]
    test_may_be_node_modules_with_node_modules_prefix_in_alias,
    "demo/node_modules/antd/lib/Button" => true
  }

  assert_may_be_node_modules! {
    #[test]
    test_may_be_node_modules_with_scope_prefix,
    "@babel/core/lib/something" => true
  }

  assert_may_be_node_modules! {
    #[test]
    test_may_be_node_modules_with_alias_prefix,
    "@/src/index.ts" => false
  }

  assert_may_be_node_modules! {
    #[test]
    test_may_be_node_modules_with_at_prefix_and_slash,
    "lodash/cloneDeep" => false
  }
}
