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
    from_str(include_str!(
      "./template_literals_template_literal_revision.json"
    ))
    .unwrap()
  });

  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_template_literals_template_literal_revision(
  v: &mut SyntaxVisitor,
) {
  v.walk_template_literal.push(walk_template_literal);
}

#[cfg(test)]
mod tests {

  //TODO test this

  use crate::{
    assert_ok_count,
    syntax::grammar::template_literals_template_literal_revision::setup_template_literals_template_literal_revision,
  };

  assert_ok_count! {
    "template_literals_template_literal_revision",
    setup_template_literals_template_literal_revision,

    should_ok_when_use_template_literals_template_literal_revision,
    r#"
      `foo`;
      `bar`;
    "#,
    2,
  }
}
