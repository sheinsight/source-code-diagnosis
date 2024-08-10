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

pub struct ArgumentsLengthVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ArgumentsLengthVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./arguments_length.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ArgumentsLengthVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ArgumentsLengthVisitor {
  fn visit_static_member_expression(
    &mut self,
    it: &oxc_ast::ast::StaticMemberExpression<'a>,
  ) {
    if let Expression::Identifier(o) = &it.object {
      if o.name == "arguments" && it.property.name == "length" {
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
      .filter(|item| item.name == "functions_arguments_length")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ArgumentsLengthVisitor::default());
    let usage = tester.analyze(
      "
function logArguments() {
    console.log(` ${arguments.length} `);
    for (let i = 0; i < arguments.length; i++) {
        console.log(`${i + 1}: ${arguments[i]}`);
    }
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }
}
