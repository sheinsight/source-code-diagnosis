use std::sync::OnceLock;

use oxc_ast::ast::BindingPatternKind;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_function(
  ctx: &mut Context,
  it: &oxc_ast::ast::Function,
  _flags: &oxc_semantic::ScopeFlags,
  _is_strict_mode: bool,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./default_parameters.json")).unwrap()
  });
  for item in &it.params.items {
    if matches!(item.pattern.kind, BindingPatternKind::AssignmentPattern(_)) {
      ctx
        .usage
        .push(CompatBox::new(item.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_default_parameters(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_default_parameters;
  use crate::assert_ok_count;

  assert_ok_count! {
    "functions_default_parameters",
    setup_default_parameters,

    should_ok_when_use_default_parameters,
    r#"
      function multiply(a, b = 1) {
        return a * b;
      }
    "#,
    1,

    should_ok_when_use_two_default_parameters,
    r#"
      function multiply(a = 1, b = 1) {
        return a * b;
      }
    "#,
    2,

    should_ok_when_not_use_default_parameters,
    r#"
      function multiply(a, b) {
        return a * b;
      }
    "#,
    0
  }
}
