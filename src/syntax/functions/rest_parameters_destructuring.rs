use std::sync::OnceLock;

use oxc_ast::ast::BindingPatternKind;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_formal_parameters(
  ctx: &mut Context,
  it: &oxc_ast::ast::FormalParameters,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./rest_parameters_destructuring.json")).unwrap()
  });
  if let Some(rest) = &it.rest {
    if matches!(rest.argument.kind, BindingPatternKind::ArrayPattern(_)) {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_rest_parameters_destructuring(
  v: &mut crate::syntax::visitor::SyntaxVisitor,
) {
  v.walk_formal_parameters.push(walk_formal_parameters);
}

#[cfg(test)]
mod tests {
  use super::setup_rest_parameters_destructuring;
  use crate::assert_ok_count;

  assert_ok_count! {
    "rest_parameters_destructuring",
    setup_rest_parameters_destructuring,

    should_ok_when_use_rest_parameters_destructuring,
    r#"
      function ignoreFirst(...[, b, c]) {
        return b + c;
      }
    "#,
    1,

    should_ok_when_not_use_rest_parameters_destructuring,
    r#"
      function ignoreFirst(...rest) {
        return rest;
      }
    "#,
    0,

    should_ok_when_not_use_rest_parameters_destructuring_with_no_parameters,
    r#"
      function ignoreFirst() {
        return 'hello';
      }
    "#,
    0,
  }
}
