use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use oxc_span::Span;
use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ArrowFunctionsTrailingCommaVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
  source_code: String,
}

impl ArrowFunctionsTrailingCommaVisitor {
  fn from_source_code(source_code: String) -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./arrow_functions_trailing_comma.json")).unwrap();
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

impl CommonTrait for ArrowFunctionsTrailingCommaVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ArrowFunctionsTrailingCommaVisitor {
  fn visit_arrow_function_expression(
    &mut self,
    it: &oxc_ast::ast::ArrowFunctionExpression<'a>,
  ) {
    let params_source_code = self.get_source_code(it.params.span);
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      if regex.is_match(params_source_code) {
        self
          .usage
          .push(CompatBox::new(it.params.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_arrow_function_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "functions_arrow_functions_trailing_comma")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let source_code = r##"
const func4 = (
  a,
  b,
  c,
) => 1;   
"##;
    let mut tester = SemanticTester::from_visitor(
      ArrowFunctionsTrailingCommaVisitor::from_source_code(
        source_code.to_string(),
      ),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
