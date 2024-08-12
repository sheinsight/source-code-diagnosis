use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use oxc_span::Span;
use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct GeneratorFunctionTrailingCommaVisitor<'a> {
  usage: Vec<CompatBox>,
  source_code: &'a str,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
}

impl<'a> GeneratorFunctionTrailingCommaVisitor<'a> {
  pub fn from_source_code(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./generator_function_trailing_comma.json"))
        .unwrap();
    Self {
      usage: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      compat: compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> CommonTrait for GeneratorFunctionTrailingCommaVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for GeneratorFunctionTrailingCommaVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_syntax::scope::ScopeFlags,
  ) {
    if it.generator && !it.r#async {
      let source_code = self.get_source_code(it.params.span);
      if let Ok(regex) = Regex::new(r",\s*\)$") {
        if regex.is_match(source_code) {
          self
            .usage
            .push(CompatBox::new(it.params.span.clone(), self.compat.clone()));
        }
      }
    }
    oxc_ast::visit::walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "generator_function_trailing_comma")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let source_code = r##"
    const foo = function* (a,b,) {
      yield 'a';
      yield 'b';
      yield 'c';
    };
        "##;
    let mut tester = SemanticTester::from_visitor(
      GeneratorFunctionTrailingCommaVisitor::from_source_code(source_code),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
