use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_property_definition(
  ctx: &mut Context,
  it: &oxc_ast::ast::PropertyDefinition,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./static.json")).unwrap());

  if it.r#static {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn walk_method_definition(
  ctx: &mut Context,
  it: &oxc_ast::ast::MethodDefinition,
) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./static.json")).unwrap());

  if it.r#static {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn walk_static_block(ctx: &mut Context, it: &oxc_ast::ast::StaticBlock) {
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./static.json")).unwrap());

  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

pub fn setup_static(visit: &mut SyntaxVisitor) {
  visit.walk_method_definition.push(walk_method_definition);
  visit
    .walk_property_definition
    .push(walk_property_definition);
  visit.walk_static_block.push(walk_static_block);
}

#[cfg(test)]
mod tests {

  use crate::{assert_ok_count, syntax::classes::r#static::setup_static};

  assert_ok_count! {
    "classes_static",
    setup_static,

    should_ok_when_use_static_method,
    r#"
      class A { static a() { } }
    "#,
    1,

    should_ok_when_use_static_property,
    r#"
      class A { static a = 1; }
    "#,
    1,

    should_ok_when_use_static_block,
    r#"
      class A { static { } }
    "#,
    1,

    should_ok_when_use_all_static,
    r#"
      class A {
        static a() { }
        static b = 1;
        static { }
      }
    "#,
    3,

    should_ok_when_use_two_static_method,
    r#"
      class A { static a() { } static b() { } }
    "#,
    2,

    should_ok_when_use_two_static_property,
    r#"
      class A { static a = 1; static b = 2; }
    "#,
    2,

    should_ok_when_use_two_static_block,
    r#"
      class A { static { } static { } }
    "#,
    2,


    should_ok_when_not_use_static,
    r#"
      class A { }
    "#,
    0,

    should_ok_when_use_static_and_not_use_static,
    r#"
      class A {
        static a() { }
        static b = 1;
        static { }
      }
      class B { }
    "#,
    3,

  }
}
