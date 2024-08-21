use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};
use serde_json5::from_str;
use std::sync::OnceLock;

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_boolean_literal(ctx: &mut Context, it: &oxc_ast::ast::BooleanLiteral) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./boolean_literals.json")).unwrap());
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_boolean_literals(v: &mut SyntaxVisitor) {
  v.walk_boolean_literal.push(walk_boolean_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, syntax::grammar::boolean_literals::setup_boolean_literals,
  };

  assert_ok_count! {
    "boolean_literals",
    setup_boolean_literals,

    should_ok_when_use_boolean_literals,
    r#"
      true;
      false;
    "#,
    2,
  }
}
