use oxc_ast::{
  ast::{BindingPatternKind, FunctionType},
  visit::walk,
  Visit,
};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct RestParametersDestructuringVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for RestParametersDestructuringVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./rest_parameters_destructuring.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for RestParametersDestructuringVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for RestParametersDestructuringVisitor {
  fn visit_formal_parameters(
    &mut self,
    it: &oxc_ast::ast::FormalParameters<'a>,
  ) {
    if let Some(rest) = &it.rest {
      if matches!(rest.argument.kind, BindingPatternKind::ArrayPattern(_)) {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_formal_parameters(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "rest_parameters_destructuring")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(
      RestParametersDestructuringVisitor::default(),
    );
    let usage = tester.analyze(
      "
function ignoreFirst(...[, b, c]) {
  return b + c;
}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
