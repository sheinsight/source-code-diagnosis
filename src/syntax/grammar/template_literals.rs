use std::sync::OnceLock;

use oxc_ast::ast::TemplateLiteral;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_template_literal(ctx: &mut Context, it: &TemplateLiteral) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./template_literals.json")).unwrap()
  });
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_template_literals(v: &mut SyntaxVisitor) {
  v.walk_template_literal.push(walk_template_literal);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::template_literals::setup_template_literals,
  };

  assert_ok_count! {
    "template_literals",
    setup_template_literals,

    should_ok_when_use_template_literals,
    r#"
      `foo`;
      `bar`;
    "#,
    2,
  }
}
