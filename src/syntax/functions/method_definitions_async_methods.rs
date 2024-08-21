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
    from_str(include_str!("./method_definitions_async_methods.json")).unwrap()
  });

  if let Some(parent) = ctx.stack.last() {
    let is_async = match parent {
      AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => {
        it.r#async && !it.generator
      }
      _ => false,
    };
    if is_async {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_method_definitions_async_methods(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_method_definitions_async_methods;
  use crate::assert_ok_count;

  assert_ok_count! {
    "async_methods",
    setup_method_definitions_async_methods,

    should_ok_when_use_async_methods,
    r#"
      const obj = {
        f: async function () {
          await somePromise;
        },
      };
    "#,
    1,

    should_ok_when_not_use_async_methods,
    r#"
      const obj = {
        f: function () {
          await somePromise;
        },
      };
    "#,
    0,

    should_ok_when_use_async_methods_with_computed_property,
    r#"
      const obj = {
        [expr]: async function () {
          await somePromise;
        },
      };
    "#,
    1,

    should_ok_when_use_async_methods_with_generator,
    r#"
      const obj = {
        f: async function* () {
          yield 1;
          yield 2;
          yield 3;
        },
      };
    "#,
    0,
  }
}
