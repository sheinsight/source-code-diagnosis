use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use oxc_span::Span;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct FunctionTrailingCommaInParametersVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
  source_code: String,
}

impl FunctionTrailingCommaInParametersVisitor {
  fn from_source_code(source_code: String) -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./function_trailing_comma_in_parameters.json"))
        .unwrap();
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

impl CommonTrait for FunctionTrailingCommaInParametersVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for FunctionTrailingCommaInParametersVisitor {
  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_semantic::ScopeFlags,
  ) {
    if matches!(it.r#type, FunctionType::FunctionDeclaration) {
      let params_source_code = self.get_source_code(it.params.span);
      if params_source_code.ends_with(",)") {
        self
          .usage
          .push(CompatBox::new(it.params.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "function_trailing_comma_in_parameters")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let source_code = "
function calcRectArea(width, height,) {
  return width * height;
}
";

    let mut tester = SemanticTester::from_visitor(
      FunctionTrailingCommaInParametersVisitor::from_source_code(
        source_code.to_string(),
      ),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
