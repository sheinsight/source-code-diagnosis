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
    from_str(include_str!("./octal_numeric_literals.json")).unwrap()
  });
  if vec!["0O", "0o"].iter().any(|item| it.raw.starts_with(item)) {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_octal_numeric_literals(v: &mut SyntaxVisitor) {
  v.walk_numeric_literal.push(walk_numeric_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::octal_numeric_literals::setup_octal_numeric_literals,
  };

  assert_ok_count! {
    "octal_numeric_literals",
    setup_octal_numeric_literals,

    should_ok_when_use_octal_numeric_literals,
    r#"
      0O755;
      0o644;
    "#,
    2,
  }
}
