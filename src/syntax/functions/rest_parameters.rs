use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_formal_parameters(
  ctx: &mut Context,
  it: &oxc_ast::ast::FormalParameters,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./rest_parameters.json")).unwrap());
  if it.rest.is_some() {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_rest_parameters(v: &mut SyntaxVisitor) {
  v.walk_formal_parameters.push(walk_formal_parameters);
}

#[cfg(test)]
mod tests {
  use super::setup_rest_parameters;
  use crate::assert_ok_count;

  assert_ok_count! {
    "rest_parameters",
    setup_rest_parameters,

    should_ok_when_use_rest_parameters,
    r#"
      function sum(a,b,...theArgs) {
        let total = 0;
        for (const arg of theArgs) {
          total += arg;
        }
        return total;
      }
    "#,
    1,

    should_ok_when_not_use_rest_parameters,
    r#"
      function sum(a,b) {
        return a + b;
      }
    "#,
    0,

    should_ok_when_not_use_rest_parameters_with_no_parameters,
    r#"
      function sum() {
        return 0;
      }
    "#,
    0,
  }
}
