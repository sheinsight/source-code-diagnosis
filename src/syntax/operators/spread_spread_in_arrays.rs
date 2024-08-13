use oxc_ast::{ast::ArrayExpressionElement, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct SpreadSpreadInArraysVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for SpreadSpreadInArraysVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./spread_spread_in_arrays.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for SpreadSpreadInArraysVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for SpreadSpreadInArraysVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_array_expression_element(
    &mut self,
    it: &oxc_ast::ast::ArrayExpressionElement<'a>,
  ) {
    if let ArrayExpressionElement::SpreadElement(arg) = it {
      self
        .usage
        .push(CompatBox::new(arg.span.clone(), self.compat.clone()));
    }
    walk::walk_array_expression_element(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "spread_in_arrays")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(SpreadSpreadInArraysVisitor::default());
    let usage = tester.analyze(
      r#"
const parts = ["shoulders", "knees"];
const lyrics = ["head", ...parts, "and", "toes"];        
"#,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
