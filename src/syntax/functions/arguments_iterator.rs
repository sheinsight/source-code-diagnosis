use std::sync::OnceLock;

use oxc_ast::AstKind;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_identifier_reference(
  ctx: &mut Context,
  it: &oxc_ast::ast::IdentifierReference,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./arguments_iterator.json")).unwrap()
  });

  if it.name == "arguments" {
    if let Some(p) = ctx.stack.last() {
      if let AstKind::ForOfStatement(_) = p {
        ctx
          .usage
          .push(CompatBox::new(it.span.clone(), compat.clone()));
      }
    }
  }
}

pub fn setup_arguments_iterator(v: &mut SyntaxVisitor) {
  v.walk_identifier_reference.push(walk_identifier_reference);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::functions::arguments_iterator::setup_arguments_iterator,
  };

  assert_ok_count! {
    "functions_arguments_iterator",
    setup_arguments_iterator,

    should_ok_when_use_arguments_iterator,
    r#"
      function f() {
        for (const letter of arguments) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    1,

    should_ok_when_not_use_arguments_iterator,
    r#"
      function f() {
        for (const letter of []) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    0,

    should_ok_when_use_arguments_iterator_in_arrow_function,
    r#"
      const f = () => {
        for (const letter of arguments) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    1,

    should_ok_when_use_arguments_iterator_in_arrow_function_with_arguments,
    r#"
      const f = (arguments) => {
        for (const letter of arguments) {
          console.log(letter);
        }
      }
      f("w", "y", "k", "o", "p");
    "#,
    1,
  }
}
