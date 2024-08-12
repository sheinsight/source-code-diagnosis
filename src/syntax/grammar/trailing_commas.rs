use oxc_ast::{
  ast::{FunctionType, ImportDeclarationSpecifier},
  visit::walk,
  AstKind, Visit,
};
use oxc_span::Span;
use regex::Regex;
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct TrailingCommasVisitor<'a> {
  usage: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  compat: Compat,
  source_code: String,
}

impl<'a> TrailingCommasVisitor<'a> {
  fn from_source_code(source_code: String) -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./trailing_commas.json")).unwrap();
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

impl<'a> CommonTrait for TrailingCommasVisitor<'a> {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for TrailingCommasVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_array_expression(&mut self, it: &oxc_ast::ast::ArrayExpression<'a>) {
    let source_code = self.get_source_code(it.span);
    if let Ok(regex) = Regex::new(r",\s*\]$") {
      if regex.is_match(source_code) {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }

    walk::walk_array_expression(self, it);
  }

  fn visit_object_expression(
    &mut self,
    it: &oxc_ast::ast::ObjectExpression<'a>,
  ) {
    let source_code = self.get_source_code(it.span);
    if let Ok(regex) = Regex::new(r",\s*\}$") {
      if regex.is_match(source_code) {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_object_expression(self, it);
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

  fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'a>) {
    let source_code = self.get_source_code(it.span);
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      if regex.is_match(source_code) {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_call_expression(self, it);
  }

  fn visit_import_declaration(
    &mut self,
    it: &oxc_ast::ast::ImportDeclaration<'a>,
  ) {
    let source_code = self.get_source_code(it.span);
    if let Ok(regex) =
      Regex::new(r##"\{\s*[^}]*,\s*}\s*from\s*['\"][^'\"]*['\"];?"##)
    {
      if regex.is_match(source_code) {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_import_declaration(self, it);
  }

  fn visit_export_named_declaration(
    &mut self,
    it: &oxc_ast::ast::ExportNamedDeclaration<'a>,
  ) {
    let source_code = self.get_source_code(it.span);
    if let Ok(regex) = Regex::new(r"export\s*\{\s*[^}]*,\s*}") {
      if regex.is_match(source_code) {
        self
          .usage
          .push(CompatBox::new(it.span.clone(), self.compat.clone()));
      }
    }
    walk::walk_export_named_declaration(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "trailing_commas")
      .count()
  }

  #[test]
  fn should_ok_when_array_expression() {
    let source_code = r##"
const arr = [
  1,
  2,
  3,
];    
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_object_literals() {
    let source_code = r##"
const object = {
  foo: "bar",
  baz: "qwerty",
  age: 42,
};  
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_function() {
    let source_code = r##"
function f(p,) {}
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_class_function() {
    let source_code = r##"
class C {
  one(a,) {}
  two(a, b,) {}
}
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }

  #[test]
  fn should_ok_when_object_function() {
    let source_code = r##"
const obj = {
  one(a,) {},
  two(a, b,) {}
};
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 2);

    assert_eq!(count, 2);
  }

  #[test]
  fn should_ok_when_function_call_expression() {
    let source_code = r##"
hello(a,b,)
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_named_import() {
    let source_code = r##"
import {
  A,
  B,
  C,
} from "D";
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_named_exported() {
    let source_code = r##"
export {
  A,
  B,
  C,
};

export { A, B, C, };

export { A as B, C as D, E as F, };
"##;
    let mut tester = SemanticTester::from_visitor(
      TrailingCommasVisitor::from_source_code(source_code.to_string()),
    );
    let usage = tester.analyze(source_code);

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 3);

    assert_eq!(count, 3);
  }
}
