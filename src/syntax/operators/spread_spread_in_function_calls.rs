use oxc_ast::{ast::Argument, visit::walk, AstKind, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct SpreadSpreadInFunctionCallsVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> Default for SpreadSpreadInFunctionCallsVisitor<'a> {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./spread_spread_in_function_calls.json")).unwrap();
    Self {
      usage,
      compat,
      parent_stack: Vec::new(),
    }
  }
}

impl<'a> CommonTrait for SpreadSpreadInFunctionCallsVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for SpreadSpreadInFunctionCallsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
    for arg in &expr.arguments {
      if let Argument::SpreadElement(arg) = arg {
        self
          .usage
          .push(CompatBox::new(arg.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_call_expression(self, expr);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "spread_in_function_calls")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      SpreadSpreadInFunctionCallsVisitor::default(),
    );
    let usage = tester.analyze(
      r#"
function myFunction(x, y, z) {}
const args = [0, 1, 2];
myFunction(...args);     
"#,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
