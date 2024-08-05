use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

pub struct GeneratorFunctionVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for GeneratorFunctionVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> GeneratorFunctionVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./generator_function.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for GeneratorFunctionVisitor<'a> {
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
    let code_seg = self.get_source_code(it.span).to_string();

    if it.generator && !it.r#async {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg.clone(),
        compat: self.compat.clone(),
      });
    }

    oxc_ast::visit::walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::{t_any, t_any_not};
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_generator_function_of_generator_function() {
    let source_code = r##"
const foo = function* () {
  yield 'a';
  yield 'b';
  yield 'c';
};
    "##;
    let allocator = Allocator::default();
    t_any(
      "generator_function",
      source_code,
      &allocator,
      GeneratorFunctionVisitor::new,
    );
  }

  #[test]
  fn should_not_exits_generator_function_of_async_generator_function() {
    let source_code = r##"
const foo = async function* () {
  yield 'a';
  yield 'b';
  yield 'c';
};
    "##;
    let allocator = Allocator::default();
    t_any_not(
      "generator_function",
      source_code,
      &allocator,
      GeneratorFunctionVisitor::new,
    );
  }
}
