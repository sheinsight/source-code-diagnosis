use std::sync::OnceLock;

use oxc_ast::ast::ClassElement;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_class_body(ctx: &mut Context, it: &oxc_ast::ast::ClassBody) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./constructor.json")).unwrap());

  it.body.iter().for_each(|item| {
    if let ClassElement::MethodDefinition(method_definition) = item {
      if let Some(name) = method_definition.key.name() {
        if name == "constructor" {
          ctx.usage.push(CompatBox::new(
            method_definition.span.clone(),
            compat.clone(),
          ));
        }
      }
    }
  });
}

pub fn setup_class_constructor(v: &mut SyntaxVisitor) {
  v.walk_class_body.push(walk_class_body);
}

#[cfg(test)]
mod tests {

  use crate::assert_ok_count;

  use super::setup_class_constructor;

  assert_ok_count! {
    "classes_constructor",
    setup_class_constructor,

    should_ok_when_use_class_constructor,
    r#"
      class A { constructor() { } }
    "#,
    1,

    should_ok_when_use_two_class_constructor,
    r#"
      class A { constructor() { } }
      class B { constructor() { } }
    "#,
    2,

    should_ok_when_not_use_constructor,
    r#"
      class H{ }
    "#,
    0
  }
}
