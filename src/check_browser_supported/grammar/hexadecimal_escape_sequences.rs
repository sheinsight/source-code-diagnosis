use std::sync::OnceLock;

use serde_json5::from_str;

use crate::check_browser_supported::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_hexadecimal_escape_sequences(
  ctx: &mut Context,
  it: &oxc_ast::ast::Directive,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./hexadecimal_escape_sequences.json")).unwrap()
  });
  if it.directive.starts_with("\\x") {
    ctx.usage.push(CompatBox::new(
      it.span.clone(),
      compat.clone(),
      ctx.file_path.clone(),
    ));
  }
}

pub fn setup_hexadecimal_escape_sequences(v: &mut SyntaxVisitor) {
  v.walk_directive.push(walk_hexadecimal_escape_sequences);
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  // TODO: Fix hexadecimal escape sequences

  // assert_ok_count! {
  //   "hexadecimal_escape_sequences",
  //   crate::syntax::grammar::hexadecimal_escape_sequences::setup_hexadecimal_escape_sequences,

  //   should_ok_when_use_hexadecimal_escape_sequences,
  //   r#""\xA9"; // "©"
  //     "xx";
  //   "#,
  //   1,

  //   should_fail_when_hexadecimal_escape_sequences,
  //   r#"
  //     "\x";
  //   "#,
  //   0,
  // }
}
