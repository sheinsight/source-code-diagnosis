use oxc_ast::{ast::FunctionType, visit::walk, Visit};
use serde_json5::from_str;

use crate::syntax::{
  common::CommonTrait,
  compat::{Compat, CompatBox},
};

pub struct TryCatchOptionalCatchBindingVisitor {
  usage: Vec<CompatBox>,
  compat: Compat,
}

impl Default for TryCatchOptionalCatchBindingVisitor {
  fn default() -> Self {
    let usage: Vec<CompatBox> = Vec::new();
    let compat: Compat =
      from_str(include_str!("./try_catch_optional_catch_binding.json"))
        .unwrap();
    Self { usage, compat }
  }
}

impl CommonTrait for TryCatchOptionalCatchBindingVisitor {
  fn get_usage(&self) -> Vec<CompatBox> {
    self.usage.clone()
  }
}

impl<'a> Visit<'a> for TryCatchOptionalCatchBindingVisitor {
  fn visit_catch_clause(&mut self, it: &oxc_ast::ast::CatchClause<'a>) {
    if it.param.is_none() {
      self
        .usage
        .push(CompatBox::new(it.span.clone(), self.compat.clone()));

      walk::walk_catch_clause(self, it);
    }
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "try_catch_optional_catch_binding")
      .count()
  }

  #[test]
  fn should_ok_when_use_optional_catch_binding() {
    let mut tester = SemanticTester::from_visitor(
      TryCatchOptionalCatchBindingVisitor::default(),
    );
    let usage = tester.analyze(
      "
try {

} catch {

}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 1);

    assert_eq!(count, 1);
  }

  #[test]
  fn should_fail_when_no_use_optional_catch_binding() {
    let mut tester = SemanticTester::from_visitor(
      TryCatchOptionalCatchBindingVisitor::default(),
    );
    let usage = tester.analyze(
      "
try {

} catch(a) {

}    
",
    );

    let count = get_async_function_count(&usage);

    assert_eq!(usage.len(), 0);

    assert_eq!(count, 0);
  }
}
