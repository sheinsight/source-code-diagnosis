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
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./public_class_fields.json")).unwrap()
  });

  if !matches!(it.key, oxc_ast::ast::PropertyKey::PrivateIdentifier(_)) {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_public_fields(v: &mut SyntaxVisitor) {
  v.walk_property_definition.push(walk_property_definition);
}

#[cfg(test)]
mod tests {
  use super::setup_public_fields;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_public_class_fields",
    setup_public_fields,

    should_ok_when_use_class_public_fields,
    r#"
      class A{ hello = 12 }
    "#,
    1,

    should_ok_when_use_class_static_public_fields,
    r#"
      class A{ static hello = 12 }
    "#,
    1,

    should_ok_when_use_class_public_fields_and_static_public_fields,
    r#"
      class A{ hello = 12; static hello = 12 }
    "#,
    2,

    should_ok_when_use_class_private_fields,
    r#"
      class A{ #hello = 12 }
    "#,
    0,

    should_ok_when_use_class_private_fields_and_static_private_fields,
    r#"
      class A{ #hello = 12; static #hello = 12 }
    "#,
    0,

    should_ok_when_use_class_public_fields_and_private_fields,
    r#"
      class A{ hello = 12; #hello = 12 }
    "#,
    1,
  }
}
