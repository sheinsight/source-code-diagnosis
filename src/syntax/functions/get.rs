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
  let compat = CONSTRUCTOR_COMPAT
    .get_or_init(|| from_str(include_str!("./get.json")).unwrap());

  if let Some(parent) = ctx.stack.last() {
    let is_get = match parent {
      AstKind::ObjectProperty(parent) => PropertyKind::Get == parent.kind,
      AstKind::MethodDefinition(parent) => {
        MethodDefinitionKind::Get == parent.kind
      }
      _ => false,
    };

    if is_get {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_get(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_get;
  use crate::assert_ok_count;

  assert_ok_count! {
    "get",
    setup_get,

    should_ok_when_use_get,
    r#"
      const obj = {
        get latest() {
          return this.log[this.log.length - 1];
        },
      };
    "#,
    1,

    should_ok_when_not_use_get,
    r#"
      const obj = {
        latest() {
          return this.log[this.log.length - 1];
        },
      };
    "#,
    0,

    should_ok_when_use_get_with_computed_property,
    r#"
      const obj = {
        get [expr]() {
          return "bar";
        },
      };
    "#,
    1,

  }
}
