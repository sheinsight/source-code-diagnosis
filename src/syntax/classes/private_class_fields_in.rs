use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};
use serde_json5::from_str;
use std::sync::OnceLock;

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_private_in_expression<'a>(
  ctx: &mut Context,
  it: &oxc_ast::ast::PrivateInExpression<'a>,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./private_class_fields_in.json")).unwrap()
  });
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_private_fields_in(v: &mut SyntaxVisitor) {
  v.walk_private_in_expression
    .push(walk_private_in_expression);
}

#[cfg(test)]
mod tests {

  use crate::assert_ok_count;

  use super::setup_private_fields_in;

  assert_ok_count! {
    "classes_private_class_fields_in",
    setup_private_fields_in,

    should_ok_when_not_use_private_in_expression,
    r#""#,
    0,

    should_ok_when_use_private_in_expression,
    r#"
      class C {
        #x;
        constructor(x) {
          this.#x = x;
        }
        static getX(obj) {
          if (#x in obj) return obj.#x;
          return "obj must be an instance of C";
        }
      }
    "#,
    1

  }
}
