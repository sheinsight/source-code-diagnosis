use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_directive(ctx: &mut Context, it: &oxc_ast::ast::Directive) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./unicode_point_escapes.json")).unwrap()
  });
  if it.directive.contains("\\u") {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_unicode_point_escapes(v: &mut SyntaxVisitor) {
  v.walk_directive.push(walk_directive);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::unicode_point_escapes::setup_unicode_point_escapes,
  };

  // TODO fix this test

  assert_ok_count! {
    "unicode_point_escapes",
    setup_unicode_point_escapes,

    should_ok_when_use_unicode_point_escapes,
    r#"
      "\uD87E\uDC04";
    "#,
    1,
  }
}
