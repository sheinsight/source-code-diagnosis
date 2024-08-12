use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct TemplateLiteralsTemplateLiteralRevisionVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for TemplateLiteralsTemplateLiteralRevisionVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!(
      "./template_literals_template_literal_revision.json"
    ))
    .unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for TemplateLiteralsTemplateLiteralRevisionVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for TemplateLiteralsTemplateLiteralRevisionVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  // TODO: Implement template_literals_template_literal_revision
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "template_literals_template_literal_revision")
      .count()
  }

  // #[test]
  // fn should_ok_when_async_generator_function_declaration() {
  //   let mut tester = SemanticTester::from_visitor(
  //     TemplateLiteralsTemplateLiteralRevisionVisitor::default(),
  //   );
  //   let usage = tester.analyze("");

  //   let count = get_async_function_count(&usage);

  //   assert_eq!(usage.len(), 1);

  //   assert_eq!(count, 1);
  // }
}
