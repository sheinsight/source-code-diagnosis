use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ExportDefaultVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ExportDefaultVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./export_default.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ExportDefaultVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ExportDefaultVisitor {
  fn visit_export_default_declaration(
    &mut self,
    it: &oxc_ast::ast::ExportDefaultDeclaration<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_export_default_declaration(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "export_default")
      .count()
  }

  #[test]
  fn should_ok_when_export_expression() {
    let mut tester =
      SemanticTester::from_visitor(ExportDefaultVisitor::default());
    let usage = tester.analyze("export default 1;");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_ok_when_export_default_function() {
    let mut tester =
      SemanticTester::from_visitor(ExportDefaultVisitor::default());
    let usage = tester.analyze("export default function() {  }");

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
