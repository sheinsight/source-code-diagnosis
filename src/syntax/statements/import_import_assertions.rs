use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ImportImportAssertionsVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ImportImportAssertionsVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./import_import_assertions.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ImportImportAssertionsVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ImportImportAssertionsVisitor {
  // fn visit_declaration(&mut self, it: &oxc_ast::ast::Declaration<'a>) {
  //   walk::walk_declaration(self, it);
  // }

  fn visit_with_clause(&mut self, it: &oxc_ast::ast::WithClause<'a>) {
    println!(
      "visit_with_clause: {}",
      it.attributes_keyword.name == "assert"
    );
    if it.attributes_keyword.name == "assert" {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    }
    walk::walk_with_clause(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "import_import_assertions")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(ImportImportAssertionsVisitor::default());
    let usage =
      tester.analyze("import json from './data.json' assert { type: 'json' };");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
