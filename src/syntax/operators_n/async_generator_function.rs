use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct AsyncGeneratorFunctionBrowserCompatMetadata {
  pub async_generator_function: Compat,
}

pub struct AsyncGeneratorFunctionVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: AsyncGeneratorFunctionBrowserCompatMetadata,
}

impl CommonTrait for AsyncGeneratorFunctionVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> AsyncGeneratorFunctionVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: AsyncGeneratorFunctionBrowserCompatMetadata =
      from_str(include_str!("./async_generator_function.json")).unwrap();
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

impl<'a> Visit<'a> for AsyncGeneratorFunctionVisitor<'a> {
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
    if it.generator && it.r#async {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self
          .browser_compat_meta_data
          .async_generator_function
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
  fn should_test() {
    let source_code = r##"
async function* (param0) {

}
    "##;
    let allocator = Allocator::default();
    t_any(
      "async_generator_function",
      source_code,
      &allocator,
      AsyncGeneratorFunctionVisitor::new,
    );
  }
}
