use oxc_ast::{ast::FunctionType, visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct TrailingCommasTrailingCommasInObjectLiteralsVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
  source_code: String,
}

impl<'a> TrailingCommasTrailingCommasInObjectLiteralsVisitor<'a> {
  fn from_source_code(source_code: String) -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!(
      "./trailing_commas_trailing_commas_in_object_literals.json"
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

impl<'a> CommonTrait
  for TrailingCommasTrailingCommasInObjectLiteralsVisitor<'a>
{
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for TrailingCommasTrailingCommasInObjectLiteralsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_expression(
    &mut self,
    it: &oxc_ast::ast::ObjectExpression<'a>,
  ) {
    if it.trailing_comma.is_some() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }

    walk::walk_object_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| {
        item.name == "trailing_commas_trailing_commas_in_object_literals"
      })
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let source_code = r##"
const obj = {
  prop1: 'value1',
  prop2: 'value2',
  prop3: 'value3',  
};    
"##;

    let mut tester = SemanticTester::from_visitor(
      TrailingCommasTrailingCommasInObjectLiteralsVisitor::from_source_code(
        source_code.to_string(),
      ),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
