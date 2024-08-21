use std::sync::OnceLock;

use oxc_ast::AstKind;
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
    from_str(include_str!("./method_definitions.json")).unwrap()
  });

  if let Some(parent) = ctx.stack.last() {
    let is_define = match parent {
      AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => true,
      _ => false,
    };
    if is_define {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_method_definitions(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_method_definitions;
  use crate::assert_ok_count;

  assert_ok_count! {
    "method_definitions",
    setup_method_definitions,

    should_ok_when_use_method_definitions,
    r#"
      const obj = {
        foo() {
          return 'bar';
        },
        bar: function () { },
      };
    "#,
    2,

    should_ok_when_not_use_method_definitions,
    r#"
      const obj = {
        foo: 'bar',
        bar: 'foo',
      };
    "#,
    0,

    should_ok_when_use_method_definitions_with_computed_property,
    r#"
      const obj = {
        [expr]() {
          return 'bar';
        },
      };
    "#,
    1,

    should_ok_when_use_method_definitions_with_async,
    r#"
      const obj = {
        async foo() {
          return 'bar';
        },
        async bar() { },
      };
    "#,
    2,
  }
}
