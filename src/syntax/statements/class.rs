use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct ClassVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for ClassVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat = from_str(include_str!("./class.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for ClassVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for ClassVisitor {
  fn visit_class(&mut self, it: &oxc_ast::ast::Class<'a>) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));
    walk::walk_class(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  #[test]
  fn should_ok_when_class_declaration() {
    let res = SemanticTester::from_visitor(ClassVisitor::default())
      .analyze("class A {}");

    let has_class = res.iter().any(|item| item.name == "class");

    assert!(has_class);
  }

  #[test]
  fn should_ok_when_class_expression() {
    let res = SemanticTester::from_visitor(ClassVisitor::default())
      .analyze("const a = class {}");

    let has_class = res.iter().any(|item| item.name == "class");

    assert!(has_class);
  }
}
