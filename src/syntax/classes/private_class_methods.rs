use std::sync::OnceLock;

use oxc_ast::ast::PropertyKey;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_method_definition(
  ctx: &mut Context,
  it: &oxc_ast::ast::MethodDefinition,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./private_class_methods.json")).unwrap()
  });

  if matches!(it.key, PropertyKey::PrivateIdentifier(_)) {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_private_method(v: &mut SyntaxVisitor) {
  v.walk_method_definition.push(walk_method_definition);
}

#[cfg(test)]
mod tests {
  use super::setup_private_method;
  use crate::assert_ok_count;

  assert_ok_count!(
    "classes_private_class_methods",
    setup_private_method,
    should_ok_when_use_private_class_methods,
    r#"
      class A {
        #private_method(){}
      }
    "#,
    1,
    should_ok_when_not_use_private_class_methods,
    r#"
    class A {
      private_method(){}
    }
  "#,
    0
  );
}
