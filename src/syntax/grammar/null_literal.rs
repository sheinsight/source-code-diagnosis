use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_null_literal(ctx: &mut Context, it: &oxc_ast::ast::NullLiteral) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./null_literal.json")).unwrap());
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_null_literal(v: &mut SyntaxVisitor) {
  v.walk_null_literal.push(walk_null_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, syntax::grammar::null_literal::setup_null_literal,
  };

  assert_ok_count! {
    "null_literal",
    setup_null_literal,

    should_ok_when_use_null_literal,
    r#"
      null;
    "#,
    1,

    should_fail_when_null_literal,
    r#"
      nul;
    "#,
    0,
  }
}
