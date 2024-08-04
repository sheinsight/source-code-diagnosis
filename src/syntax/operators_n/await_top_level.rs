use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

pub struct AwaitVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for AwaitVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> AwaitVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./await_top_level.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      compat: compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }

  fn is_top_level_await(&self) -> bool {
    match self.parent_stack.last() {
      Some(AstKind::Program(_))
      | Some(AstKind::ExportDefaultDeclaration(_))
      | Some(AstKind::ImportDeclaration(_))
      | Some(AstKind::ExpressionStatement(_))
      | Some(AstKind::VariableDeclarator(_))
      | Some(AstKind::ReturnStatement(_))
      | Some(AstKind::IfStatement(_)) => true,
      _ => false,
    }
  }
}

impl<'a> Visit<'a> for AwaitVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_await_expression(&mut self, it: &oxc_ast::ast::AwaitExpression<'a>) {
    if self.is_top_level_await() {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.compat.clone(),
      });
    }
    oxc_ast::visit::walk::walk_await_expression(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exist_top_level_await() {
    let source_code = r##"
const response = await fetch('https://api.example.com/data');
const data = await response.json();
"##;
    let allocator = Allocator::default();
    t_any(
      "await_top_level",
      source_code,
      &allocator,
      AwaitVisitor::new,
    );
  }
}
