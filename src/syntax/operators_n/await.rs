use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;

use crate::syntax::{compat::CompatBox, operators::Operators};

use super::common_trait::CommonTrait;

pub struct AwaitVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  operators: Operators,
}

impl CommonTrait for AwaitVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> AwaitVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let operators_str = include_str!("./browser_compat_data/operators.json");
    let operators: Operators = serde_json::from_str(operators_str).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      operators: operators,
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
        compat: self.operators.r#await_top_level.clone(),
      });
    } else {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.operators.r#await.clone(),
      });
    }

    oxc_ast::visit::walk::walk_await_expression(self, it);
  }
}

#[cfg(test)]
mod tests {
  use oxc_allocator::Allocator;
  use oxc_parser::Parser;
  use oxc_span::SourceType;

  use crate::syntax::operators_n::t::t_any;

  use super::*;

  fn t<F>(source_code: &str, assert_fn: F)
  where
    F: Fn(Vec<CompatBox>),
  {
    let mut visitor = AwaitVisitor::new(&source_code);
    let allocator = Allocator::default();
    let source_type = SourceType::default();
    let ret = Parser::new(&allocator, source_code, source_type).parse();
    visitor.visit_program(&ret.program);
    assert_fn(visitor.cache);
  }

  #[test]
  fn should_exist_await() {
    let source_code = r##"
async function f3() {
  const y = await 20;
  console.log(y); // 20

  const obj = {};
  console.log((await obj) === obj); // true
}
f3();
"##;
    let allocator = Allocator::default();
    t_any("await", source_code, &allocator, AwaitVisitor::new);
  }

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
