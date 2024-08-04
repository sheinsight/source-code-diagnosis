use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct FunctionBrowserCompatMetadata {
  pub function: Compat,
  pub function_trailing_comma: Compat,
}

pub struct FunctionVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: FunctionBrowserCompatMetadata,
}

impl CommonTrait for FunctionVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> FunctionVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: FunctionBrowserCompatMetadata =
      from_str(include_str!("./function.json")).unwrap();
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

impl<'a> Visit<'a> for FunctionVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_syntax::scope::ScopeFlags,
  ) {
    if !it.r#async && !it.generator {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.browser_compat_meta_data.function.clone(),
      });
    }

    let params_span = it.params.span;
    let code_seg = self.get_source_code(params_span);
    if code_seg.ends_with(",)") {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg.to_string(),
        compat: self
          .browser_compat_meta_data
          .function_trailing_comma
          .clone(),
      });
    }

    oxc_ast::visit::walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_function_of_function_declaration() {
    let source_code = r##"
const getRectArea = function (width, height) {

};
    "##;
    let allocator = Allocator::default();
    t_any("function", source_code, &allocator, FunctionVisitor::new);
  }

  #[test]
  fn should_exits_function_trailing_comma_of_function_declaration() {
    let source_code = r##"
const getRectArea = function (width, height,) {

};
    "##;
    let allocator = Allocator::default();
    t_any(
      "function_trailing_comma",
      source_code,
      &allocator,
      FunctionVisitor::new,
    );
  }
}
