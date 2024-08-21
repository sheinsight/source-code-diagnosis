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
    .get_or_init(|| from_str(include_str!("./set.json")).unwrap());
  if let Some(parent) = ctx.stack.last() {
    let is_set = match parent {
      AstKind::ObjectProperty(parent) => PropertyKind::Set == parent.kind,
      AstKind::MethodDefinition(parent) => {
        MethodDefinitionKind::Set == parent.kind
      }
      _ => false,
    };
    if is_set {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_set(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_set;
  use crate::assert_ok_count;

  assert_ok_count! {
    "set",
    setup_set,

    should_ok_when_use_set,
    r#"
      const language = {
        set current(name) {
          this.log.push(name);
        },
        log: [],
      };
    "#,
    1,

    should_ok_when_not_use_set,
    r#"
      const language = {
        current(name) {
          this.log.push(name);
        },
        log: [],
      };
    "#,
    0,

    should_ok_when_use_set_with_async,
    r#"
      const language = {
        async set current(name) {
          this.log.push(name);
        },
        log: [],
      };
    "#,
    0,
  }
}
