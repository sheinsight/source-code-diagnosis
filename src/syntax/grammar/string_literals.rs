use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_string_literal(ctx: &mut Context, it: &oxc_ast::ast::StringLiteral) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./string_literals.json")).unwrap());
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_string_literals(v: &mut SyntaxVisitor) {
  v.walk_string_literal.push(walk_string_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, syntax::grammar::string_literals::setup_string_literals,
  };

  assert_ok_count! {
    "string_literals",
    setup_string_literals,

    should_ok_when_use_string_literals,
    r#"
      'foo';
      "bar";
    "#,
    2,
  }
}
