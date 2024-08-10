use oxc_ast::{visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct PrivateClassFieldsInVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for PrivateClassFieldsInVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./private_class_fields_in.json")).unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for PrivateClassFieldsInVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for PrivateClassFieldsInVisitor {
  fn visit_private_in_expression(
    &mut self,
    it: &oxc_ast::ast::PrivateInExpression<'a>,
  ) {
    self
      .usage
      .push(CompatBox::new(it.span.clone(), self.compat.clone()));

    walk::walk_private_in_expression(self, it);
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "classes_private_class_fields_in")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    let mut tester =
      SemanticTester::from_visitor(PrivateClassFieldsInVisitor::default());
    let usage = tester.analyze(
      r##"
class C {
  #x;
  constructor(x) {
    this.#x = x;
  }
  static getX(obj) {
    if (#x in obj) return obj.#x;

    return "obj must be an instance of C";
  }
}
console.log(C.getX(new C("foo"))); // "foo"
console.log(C.getX(new C(0.196))); // 0.196
console.log(C.getX(new C(new Date()))); // the current date and time
console.log(C.getX({})); // "obj must be an instance of C"
"##,
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }
}
