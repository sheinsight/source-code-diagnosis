use std::sync::OnceLock;

use oxc_ast::ast::PropertyKey;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
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

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::visitor::SyntaxVisitor};

  fn setup_method_definition(v: &mut SyntaxVisitor) {
    v.walk_method_definition.push(super::walk_method_definition);
  }

  assert_ok_count!(
    "classes_private_class_methods",
    setup_method_definition,
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
