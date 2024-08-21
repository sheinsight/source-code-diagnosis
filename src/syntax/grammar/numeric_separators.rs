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
    from_str(include_str!("./numeric_separators.json")).unwrap()
  });
  if it.raw.contains("_") {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_numeric_separators(v: &mut SyntaxVisitor) {
  v.walk_numeric_literal.push(walk_numeric_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::numeric_separators::setup_numeric_separators,
  };

  assert_ok_count! {
    "numeric_separators",
    setup_numeric_separators,

    should_ok_when_use_numeric_separators,
    r#"
      1_000_000_000_000;
    "#,
    1,
  }
}
