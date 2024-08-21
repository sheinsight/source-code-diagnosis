use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_directive(ctx: &mut Context, it: &oxc_ast::ast::Directive) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./boolean_literals.json")).unwrap());
  if it.directive.starts_with("\\\\u") {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_unicode_escape_sequences(v: &mut SyntaxVisitor) {
  v.walk_directive.push(walk_directive);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::unicode_escape_sequences::setup_unicode_escape_sequences,
  };

  // TODO fix this test

  // assert_ok_count! {
  //   "unicode_escape_sequences",
  //   setup_unicode_escape_sequences,

  //   should_ok_when_use_unicode_escape_sequences,
  //   r#"
  //     "\\u00A9"; // "©" (U+A9)
  //   "#,
  //   1,
  // }
}
