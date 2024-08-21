use std::sync::OnceLock;

use oxc_ast::{
  ast::{MethodDefinitionKind, PropertyKind},
  AstKind,
};
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_function(
  ctx: &mut Context,
  it: &oxc_ast::ast::Function,
  _flags: &oxc_semantic::ScopeFlags,
  _is_strict_mode: bool,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./get_computed_property_names.json")).unwrap()
  });

  if let Some(parent) = ctx.stack.last() {
    let is_get_in_computed = match parent {
      AstKind::ObjectProperty(parent) => {
        PropertyKind::Get == parent.kind && parent.computed
      }
      AstKind::MethodDefinition(parent) => {
        MethodDefinitionKind::Get == parent.kind && parent.computed
      }
      _ => false,
    };

    if is_get_in_computed {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_get_computed_property_names(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_get_computed_property_names;
  use crate::assert_ok_count;

  assert_ok_count! {
    "get_computed_property_names",
    setup_get_computed_property_names,

    should_ok_when_use_get_computed_property_names,
    r#"
      const obj = {
        get [expr]() {
          return "bar";
        },
      };
    "#,
    1,

    should_ok_when_not_use_get_computed_property_names,
    r#"
      const obj = {
        get foo() {
          return "bar";
        },
      };
    "#,
    0


  }
}
