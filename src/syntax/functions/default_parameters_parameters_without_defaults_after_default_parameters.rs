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

pub struct DefaultParametersParametersWithoutDefaultsAfterDefaultParametersVisitor
{
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default
  for DefaultParametersParametersWithoutDefaultsAfterDefaultParametersVisitor
{
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./default_parameters_parameters_without_defaults_after_default_parameters.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait
  for DefaultParametersParametersWithoutDefaultsAfterDefaultParametersVisitor
{
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a>
  for DefaultParametersParametersWithoutDefaultsAfterDefaultParametersVisitor
{
  fn visit_formal_parameters(
    &mut self,
    it: &oxc_ast::ast::FormalParameters<'a>,
  ) {
    let mut flag: i32 = 0;
    for item in &it.items {
      match item.pattern.kind {
        BindingPatternKind::AssignmentPattern(_) => {
          flag = 1;
        }
        BindingPatternKind::BindingIdentifier(_)
        | BindingPatternKind::ObjectPattern(_)
        | BindingPatternKind::ArrayPattern(_) => {
          if flag == 1 {
            flag = -1;
          } else {
            flag = 0;
          }
        }
      }

      if flag == -1 {
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
    usage.iter().filter(|item| item.name == "functions_default_parameters_parameters_without_defaults_after_default_parameters").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester = SemanticTester::from_visitor(DefaultParametersParametersWithoutDefaultsAfterDefaultParametersVisitor::default());
    let usage = tester.analyze("function example(a = 1, b) {}");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
