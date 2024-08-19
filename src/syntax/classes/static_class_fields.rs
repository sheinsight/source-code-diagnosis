use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_property_definition(
  ctx: &mut Context,
  it: &oxc_ast::ast::PropertyDefinition,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./static_class_fields.json")).unwrap()
  });

  if it.r#static {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::visitor::SyntaxVisitor};

  use super::walk_property_definition;

  fn setup_property_definition(v: &mut SyntaxVisitor) {
    v.walk_property_definition.push(walk_property_definition);
  }

  assert_ok_count!(
    "classes_static_class_fields",
    setup_property_definition,
    should_ok_when_use_class_static_fields,
    r#"
      class A{ static hello = 12 }
    "#,
    1,
    should_ok_when_use_class_static_fields_and_instance_static_fields,
    r#"
      class A{ static hello = 12; hello = 12 }
    "#,
    1,
    should_ok_when_use_class_instance_fields,
    r#"
      class A{ hello = 12 }
    "#,
    0,
    should_ok_when_use_class_instance_fields_and_instance_static_fields,
    r#"
      class A{ hello = 12; static hello = 12 }
    "#,
    1,
  );
}
