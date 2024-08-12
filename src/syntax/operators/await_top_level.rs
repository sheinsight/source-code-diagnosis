use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct AwaitTopLevelVisitor<'a> {
  usage: Vec<CompatBox>,
  source_code: &'a str,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> AwaitTopLevelVisitor<'a> {
  pub fn from_source_code(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./await_top_level.json")).unwrap();
    Self {
      usage: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
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

// impl<'a> Default for AwaitTopLevelVisitor<'a> {
//   fn default() -> Self {
//     let usage: Vec<CompatBox> = Vec::new();
//     let compat: Compat =
//       from_str(include_str!("./await_top_level.json")).unwrap();
//     Self {
//       usage,
//       compat,
//       parent_stack: Vec::new(),
//     }
//   }
// }

impl<'a> CommonTrait for AwaitTopLevelVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for AwaitTopLevelVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_await_expression(&mut self, it: &oxc_ast::ast::AwaitExpression<'a>) {
    if self.is_top_level_await() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
    oxc_ast::visit::walk::walk_await_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "await_top_level")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let source_code = r#"
const response = await fetch('https://api.example.com/data');
const data = await response.json();
"#;
    let mut tester = SemanticTester::from_visitor(
      AwaitTopLevelVisitor::from_source_code(source_code),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
