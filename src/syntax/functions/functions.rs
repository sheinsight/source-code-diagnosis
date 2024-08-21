use std::sync::OnceLock;

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
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./functions.json")).unwrap());

  if it.is_declaration() {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_functions(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_functions;
  use crate::assert_ok_count;

  assert_ok_count! {
    "functions",
    setup_functions,

    should_ok_when_use_function_declaration,
    r#"
      function formatNumber(num) {
        return num.toFixed(2);
      }
    "#,
    1,

    should_ok_when_use_two_function_declaration,
    r#"
      function formatNumber(num) {
        return num.toFixed(2);
      }
      function formatString(str) {
        return str.toUpperCase();
      }
    "#,
    2,

    should_ok_when_not_use_function_declaration,
    r#"
      const formatNumber = function(num) {
        return num.toFixed(2);
      };
    "#,
    0

  }
}
