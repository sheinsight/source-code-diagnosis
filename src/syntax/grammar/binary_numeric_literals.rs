use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_numeric_literal(ctx: &mut Context, it: &oxc_ast::ast::NumericLiteral) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./binary_numeric_literals.json")).unwrap()
  });
  if vec!["0b", "0B"]
    .iter()
    .any(|item| it.raw.starts_with(*item))
  {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_binary_numeric_literals(v: &mut SyntaxVisitor) {
  v.walk_numeric_literal.push(walk_numeric_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::binary_numeric_literals::setup_binary_numeric_literals,
  };

  assert_ok_count! {
    "binary_numeric_literals",
    setup_binary_numeric_literals,

    should_ok_when_use_binary_numeric_literals,
    r#"
      0b10000000000000000000000000000000 // 2147483648
      0b01111111100000000000000000000000 // 2139095040
      0B00000000011111111111111111111111 // 8388607
    "#,
    3,
  }
}
