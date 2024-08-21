use std::{ops::Not as _, sync::OnceLock};

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_numeric_literal(
  ctx: &mut Context,
  it: &oxc_ast::ast::NumericLiteral,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./decimal_numeric_literals.json")).unwrap()
  });
  if it.raw.starts_with("0").not() {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_decimal_numeric_literals(v: &mut SyntaxVisitor) {
  v.walk_numeric_literal.push(walk_numeric_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::decimal_numeric_literals::setup_decimal_numeric_literals,
  };

  assert_ok_count! {
    "decimal_numeric_literals",
    setup_decimal_numeric_literals,

    should_ok_when_use_decimal_numeric_literals,
    r#"
      1234567890;
      42;
    "#,
    2,

    should_fail_when_decimal_numeric_literals,
    r#"
      0B42;
    "#,
    0,
  }
}
