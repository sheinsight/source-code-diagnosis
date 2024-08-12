use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use oxc_span::Span;
use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct TrailingCommasTrailingCommasInFunctionsVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: String,
  compat: Compat,
}

impl<'a> TrailingCommasTrailingCommasInFunctionsVisitor<'a> {
  fn from_source_code(source_code: String) -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!(
      "./trailing_commas_trailing_commas_in_functions.json"
    ))
    .unwrap();
    Self {
      usage,
      compat,
      source_code,
      parent_stack: Vec::new(),
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> CommonTrait for TrailingCommasTrailingCommasInFunctionsVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for TrailingCommasTrailingCommasInFunctionsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_semantic::ScopeFlags,
  ) {
    let source_code = self.get_source_code(it.params.span);
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      if regex.is_match(source_code) {
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
    usage.iter().filter(|item| item.name == "__tmp__").count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let source_code = r##"
function myFunction(
  param1,
  param2,
) {

}    
"##;

    let mut tester = SemanticTester::from_visitor(
      TrailingCommasTrailingCommasInFunctionsVisitor::from_source_code(
        source_code.to_string(),
      ),
    );

    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
