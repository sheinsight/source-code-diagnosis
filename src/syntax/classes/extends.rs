use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_class(ctx: &mut Context, it: &oxc_ast::ast::Class) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./extends.json")).unwrap());
  if it.super_class.is_some() {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

#[cfg(test)]
mod tests {

  use crate::{assert_ok_count, syntax::visitor::SyntaxVisitor};

  use super::walk_class;

  fn setup_class_extends(v: &mut SyntaxVisitor) {
    v.walk_class.push(walk_class);
  }

  assert_ok_count! {
    "classes_extends",
    setup_class_extends,

    should_ok_when_use_class_extends,
    r#"
      class A extends B { }
    "#,
    1,

    should_ok_when_use_two_class_extends,
    r#"
      class A extends B { }
      class C extends D { }
    "#,
    2,

    should_ok_when_not_use_extends,
    r#"
      class H{ }
    "#,
    0
  }
}
