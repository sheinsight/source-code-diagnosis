use oxc_ast::{
  ast::{BindingPattern, BindingPatternKind, FunctionType},
  visit::walk,
  Visit,
};
use oxc_span::Span;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

fn has_assignment_pattern(binding_pattern: &BindingPattern) -> bool {
  struct Checker {
    found: bool,
  }

  impl<'a> Visit<'a> for Checker {
    fn visit_assignment_pattern(
      &mut self,
      _pattern: &oxc_ast::ast::AssignmentPattern<'a>,
    ) {
      self.found = true;
    }
  }

  let mut checker = Checker { found: false };
  checker.visit_binding_pattern(binding_pattern);
  checker.found
}

pub struct DefaultParametersDestructuredParameterWithDefaultValueAssignmentVisitor
{
  usage: Vec<CompatBox>,
  source_code: String,
  compat: Compat,
}

impl DefaultParametersDestructuredParameterWithDefaultValueAssignmentVisitor {
  fn from_source_code(source_code: String) -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./default_parameters_destructured_parameter_with_default_value_assignment.json")).unwrap();
    Self {
      usage,
      compat,
      source_code,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl CommonTrait
  for DefaultParametersDestructuredParameterWithDefaultValueAssignmentVisitor
{
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a>
  for DefaultParametersDestructuredParameterWithDefaultValueAssignmentVisitor
{
  fn visit_formal_parameter(&mut self, it: &oxc_ast::ast::FormalParameter<'a>) {
    if let BindingPatternKind::AssignmentPattern(pattern) = &it.pattern.kind {
      if has_assignment_pattern(&pattern.left) {
        self
          .usage
          .push(CompatBox::new(it.span, self.compat.clone()));
      }
    }
    walk::walk_formal_parameter(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage.iter().filter(|item| item.name == "functions_default_parameters_destructured_parameter_with_default_value_assignment").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let source_code = r##"
function preFilledObject({ z = 3 } = {}) {
  return z;
}
"##;
    let mut tester = SemanticTester::from_visitor(DefaultParametersDestructuredParameterWithDefaultValueAssignmentVisitor::from_source_code(source_code.to_string()));
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
