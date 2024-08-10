use oxc_ast::{
  ast::{Expression, FunctionType},
  visit::walk,
  Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ArgumentsCalleeVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ArgumentsCalleeVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./arguments_callee.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ArgumentsCalleeVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ArgumentsCalleeVisitor {
  fn visit_static_member_expression(
    &mut self,
    it: &oxc_ast::ast::StaticMemberExpression<'a>,
  ) {
    if let Expression::Identifier(o) = &it.object {
      if o.name == "arguments" && it.property.name == "callee" {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }

    walk::walk_static_member_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "functions_arguments_callee")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ArgumentsCalleeVisitor::default());
    let usage = tester.analyze(
      "
function factorial(n) {
    return n <= 1 ? 1 : n * arguments.callee(n - 1);
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
