use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_reg_exp_literal(ctx: &mut Context, it: &oxc_ast::ast::RegExpLiteral) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./regular_expression_literals.json")).unwrap()
  });
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_regular_expression_literals(v: &mut SyntaxVisitor) {
  v.walk_reg_exp_literal.push(walk_reg_exp_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::regular_expression_literals::setup_regular_expression_literals,
  };

  assert_ok_count! {
    "regular_expression_literals",
    setup_regular_expression_literals,

    should_ok_when_use_regular_expression_literals,
    r#"
      /ab+c/g;
      /[/]/;
    "#,
    2,
  }
}
