use std::path::Path;

use oxc_allocator::Allocator;
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use utils::{
  glob_by_semantic, source_type_from_path, GlobErrorHandler, GlobSuccessHandler,
};

use crate::{
  browser_versions::BrowserVersions, classes, functions, grammar, operators,
  statements, target::Target, CompatBox, CompatHandler,
};

pub struct CompatChecker {
  handlers: Vec<Box<dyn CompatHandler>>,
}

impl CompatChecker {
  pub fn new(target: Target) -> anyhow::Result<Self> {
    let versions = BrowserVersions::new(target)?;
    let handlers = Self::setup_handlers(&versions);
    Ok(Self { handlers })
  }

  fn setup_handlers(versions: &BrowserVersions) -> Vec<Box<dyn CompatHandler>> {
    vec![
      classes::setup(),
      functions::setup(),
      grammar::setup(),
      operators::setup(),
      statements::setup(),
    ]
    .into_iter()
    .flat_map(|setup| setup.into_iter())
    .filter(|item| Self::is_compatible(versions, item))
    .collect()
  }

  fn is_compatible(
    versions: &BrowserVersions,
    item: &Box<dyn CompatHandler>,
  ) -> bool {
    let compat = item.get_compat();
    let support = &compat.support;
    versions.contains_version("chrome", &support.chrome)
      || versions.contains_version("firefox", &support.firefox)
      || versions.contains_version("safari", &support.safari)
      || versions.contains_version("edge", &support.edge)
      || versions.contains_version("node", &support.node)
  }

  pub fn check_source(
    &self,
    source_code: &str,
    file_path: &str,
  ) -> anyhow::Result<Vec<CompatBox>> {
    let allocator = Allocator::default();

    let source_type = source_type_from_path(&Path::new(&file_path));

    let parser = Parser::new(&allocator, &source_code, source_type);

    let parse = parser.parse();

    let program = allocator.alloc(&parse.program);

    let semantic_return = SemanticBuilder::new()
      .with_check_syntax_error(false)
      // TODO 很多场景下是不需要开启的，只有 oxlint 下需要开启，这可能对性能会产生一定的影响
      .with_cfg(true)
      .build(program);

    let nodes = semantic_return.semantic.nodes();

    let nodes = nodes
      .iter()
      .map(|item| {
        return self
          .handlers
          .iter()
          .filter_map(|compat_handler| {
            if compat_handler.handle(
              semantic_return.semantic.source_text(),
              item,
              nodes,
            ) {
              let ast_node =
                beans::AstNode::with_source_and_ast_node(&source_code, item);
              Some(CompatBox::new(
                ast_node,
                compat_handler.get_compat().clone(),
                file_path.to_string(),
              ))
            } else {
              None
            }
          })
          .collect::<Vec<_>>();
      })
      .flatten()
      .collect::<Vec<_>>();

    Ok(nodes)
  }

  pub fn check_glob(
    &self,
    args: utils::GlobArgs,
  ) -> anyhow::Result<Vec<CompatBox>> {
    let responses = glob_by_semantic(
      |GlobSuccessHandler {
         relative_path,
         semantic,
         ..
       }| {
        let mut used: Vec<CompatBox> = Vec::new();
        for node in semantic.nodes().iter() {
          for compat_handler in self.handlers.iter() {
            if compat_handler.handle(
              semantic.source_text(),
              node,
              semantic.nodes(),
            ) {
              let ast_node = beans::AstNode::with_source_and_ast_node(
                semantic.source_text(),
                node,
              );

              used.push(CompatBox::new(
                ast_node,
                compat_handler.get_compat().clone(),
                relative_path.to_string(),
              ));
            }
          }
        }
        Some(used)
      },
      |GlobErrorHandler { .. }| None,
      &args,
    )?
    .into_iter()
    .flatten()
    .collect();

    Ok(responses)
  }
}
