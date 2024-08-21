use std::sync::OnceLock;

use oxc_ast::ast::Expression;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_static_member_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::StaticMemberExpression,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./arguments_callee.json")).unwrap());
  if let Expression::Identifier(o) = &it.object {
    if o.name == "arguments" && it.property.name == "callee" {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_arguments_callee(v: &mut SyntaxVisitor) {
  v.walk_static_member_expression
    .push(walk_static_member_expression);
}

#[cfg(test)]
mod tests {

  use crate::assert_ok_count;

  assert_ok_count! {
    "functions_arguments_callee",
    super::setup_arguments_callee,

    should_ok_when_use_arguments_callee,
    r#"
      function factorial(n) {
          return n <= 1 ? 1 : n * arguments.callee(n - 1);
      }
    "#,
    1,

    should_ok_when_not_use_arguments_callee,
    r#"
      function factorial(n) {
          return n <= 1 ? 1 : n * n;
      }
    "#,
    0,

    should_ok_when_use_arguments_callee_in_arrow_function,
    r#"
      const factorial = (n) => {
          return n <= 1 ? 1 : n * arguments.callee(n - 1);
      }
    "#,
    1,

  }
}
