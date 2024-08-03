use std::{marker::PhantomData, ops::Not};

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct ImportBrowserCompatMetadata {
  r#import: Compat,
  r#import_options_parameter: Compat,
}

pub struct ImportVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: ImportBrowserCompatMetadata,
}

impl CommonTrait for ImportVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ImportVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: ImportBrowserCompatMetadata =
      from_str(include_str!("./import.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      browser_compat_meta_data: browser_compat_meta_data,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for ImportVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_import_expression(
    &mut self,
    expr: &oxc_ast::ast::ImportExpression<'a>,
  ) {
    self.cache.push(CompatBox {
      start: expr.span.start,
      end: expr.span.end,
      code_seg: self.get_source_code(expr.span).to_string(),
      compat: self.browser_compat_meta_data.import.clone(),
    });
    if expr.arguments.is_empty().not() {
      self.cache.push(CompatBox {
        start: expr.span.start,
        end: expr.span.end,
        code_seg: self.get_source_code(expr.span).to_string(),
        compat: self
          .browser_compat_meta_data
          .import_options_parameter
          .clone(),
      });
    }
    oxc_ast::visit::walk::walk_import_expression(self, expr);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exist_import() {
    let source_code = r#"
import(moduleName);
    "#;

    let allocator = Allocator::default();
    t_any("import", source_code, &allocator, ImportVisitor::new)
  }

  #[test]
  fn should_exist_import_options_parameter() {
    let source_code = r#"
import("./module.js", { a: 1 })
	.then((module) => {
		console.log(module.default);
	})
	.catch((error) => {
		console.error("Error importing module:", error);
	});
"#;

    let allocator = Allocator::default();
    t_any(
      "import_options_parameter",
      source_code,
      &allocator,
      ImportVisitor::new,
    );
  }
}
