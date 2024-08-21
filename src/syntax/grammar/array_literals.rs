use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_array_expression(
  ctx: &mut Context,
  it: &oxc_ast::ast::ArrayExpression,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./array_literals.json")).unwrap());
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_array_literals(v: &mut SyntaxVisitor) {
  v.walk_array_expression.push(walk_array_expression);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, syntax::grammar::array_literals::setup_array_literals,
  };

  assert_ok_count! {
    "array_literals",
    setup_array_literals,

    should_ok_when_use_array_literals,
    r#"
      const fruits = ["Apple", "Banana"];
    "#,
    1,

    should_ok_when_use_two_array_literals,
    r#"
      const fruits = ["Apple", "Banana"];
      const vegetables = ["Carrot", "Potato"];
    "#,
    2,
  }
}
