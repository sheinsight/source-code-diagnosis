use std::{
  fs::read_to_string,
  path::Path,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
  },
};

use beans::AstNode;
use bimap::BiMap;
use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_resolver::{AliasValue, ResolveContext, ResolveOptions, Resolver};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use utils::{glob_by, source_type_from_path, GlobArgs};

use crate::model::{Args, Edge, Graphics};

pub fn get_graph(args: Args) -> anyhow::Result<Graphics> {
  let cwd_path = Path::new(&args.cwd);

  let glob_args = GlobArgs {
    cwd: args.cwd.clone(),
    pattern: args.pattern,
    ignore: args.ignore,
  };

  let resolver_alias: Vec<(String, Vec<AliasValue>)> = args
    .alias
    .into_iter()
    .map(|(key, values)| {
      (key, values.into_iter().map(AliasValue::Path).collect())
    })
    .collect();

  let resolve_options = ResolveOptions {
    alias: resolver_alias,
    modules: args.modules,
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
    extensions: vec![
      ".js".into(),
      ".jsx".into(),
      ".ts".into(),
      ".tsx".into(),
      ".json".into(),
      ".node".into(),
      ".css".into(),
      ".scss".into(),
      ".less".into(),
      ".d.ts".into(),
    ],
    ..ResolveOptions::default()
  };

  let resolver = Resolver::new(resolve_options);

  let shared_context =
    Arc::new(parking_lot::Mutex::new(ResolveContext::default()));

  let syntax_errors = Arc::new(parking_lot::Mutex::new(Vec::new()));

  let responses = glob_by(
    |path| {
      let allocator = Allocator::default();

      if let Ok(source_text) = read_to_string(path) {
        let source_type = source_type_from_path(path);
        let ret = Parser::new(&allocator, &source_text, source_type).parse();

        let source_path = pathdiff::diff_paths(path, cwd_path)?;

        let source_dir_path = path.parent();

        let source = source_path.to_string_lossy().to_string();

        if !ret.errors.is_empty() {
          syntax_errors
            .lock()
            .push(path.to_string_lossy().to_string());
          return None;
        }

        let res: Vec<Edge> = ret
          .module_record
          .import_entries
          .par_iter()
          .filter_map(|item| {
            let item = &item.module_request;
            let name_str = item.name.to_string();

            if let Some(source_dir_path) = source_dir_path {
              let resolved = resolver.resolve_with_context(
                &source_dir_path,
                &name_str,
                &mut shared_context.lock(),
              );

              if let Ok(resolved) = resolved {
                let target_path =
                  pathdiff::diff_paths(resolved.full_path(), cwd_path)?;

                let target = target_path.to_string_lossy().to_string();

                let is_node_modules = target.contains("node_modules");

                let module_name = if is_node_modules {
                  Some(name_str)
                } else {
                  None
                };

                return Some(Edge {
                  source: source.clone(),
                  target: target,
                  missing: false,
                  target_module_name: module_name,
                  ast_node: AstNode::with_source_and_span(
                    &source_text,
                    &item.span,
                  ),
                });
              } else {
                if might_be_node_modules(&name_str) {
                  let main_module = get_main_module_name(&name_str);
                  return Some(Edge {
                    source: source.clone(),
                    target: name_str,
                    missing: true,
                    target_module_name: Some(main_module),
                    ast_node: AstNode::with_source_and_span(
                      &source_text,
                      &item.span,
                    ),
                  });
                }

                return Some(Edge {
                  source: source.clone(),
                  target: name_str,
                  missing: true,
                  target_module_name: None,
                  ast_node: AstNode::with_source_and_span(
                    &source_text,
                    &item.span,
                  ),
                });
              }
            }

            return Some(Edge {
              source: source.clone(),
              target: name_str,
              missing: true,
              target_module_name: None,
              ast_node: AstNode::with_source_and_span(&source_text, &item.span),
            });
          })
          .collect();
        Some(res)
      } else {
        None
      }
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

      let source_id = get_or_create_id(&x.source);
      let target_id = get_or_create_id(&x.target);
      let module_id = if let Some(module_name) = &x.target_module_name {
        Some(get_or_create_id(module_name))
      } else {
        None
      };

      Some(Edge {
        source: source_id,
        target: target_id,
        target_module_name: module_id,
        ..x.clone()
      })
    })
    .collect::<Vec<_>>();

  let syntax_errors = syntax_errors.lock().to_vec();

  let bi_map = bi_map.lock();

  Ok(Graphics {
    dictionaries: bi_map.iter().map(|(k, v)| (k.clone(), v.clone())).collect(),
    graph: edges,
    invalid_syntax_files: syntax_errors.clone(),
    syntax_errors: syntax_errors,
  })
}

fn might_be_node_modules(target: &str) -> bool {
  const LOCAL_PATTERNS: [&str; 5] = ["./", "../", "/", "node_modules", "@/"];

  if target.starts_with('@') {
    return !target.starts_with("@/");
  }

  !LOCAL_PATTERNS
    .iter()
    .any(|pattern| target.contains(pattern))
}

fn get_main_module_name(module_name: &str) -> String {
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
