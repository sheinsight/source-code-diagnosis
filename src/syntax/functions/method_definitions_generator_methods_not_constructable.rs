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
    from_str(include_str!(
      "./method_definitions_generator_methods_not_constructable.json"
    ))
    .unwrap()
  });

  if let Some(parent) = ctx.stack.last() {
    let is_generator = match parent {
      AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => {
        !it.r#async && it.generator
      }
      _ => false,
    };
    if is_generator {
      ctx
        .usage
        .push(CompatBox::new(it.span.clone(), compat.clone()));
    }
  }
}

pub fn setup_setup_method_definitions_generator_methods_not_constructable(
  v: &mut SyntaxVisitor,
) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use super::setup_setup_method_definitions_generator_methods_not_constructable;
  use crate::assert_ok_count;

  assert_ok_count! {
    "generator_methods_not_constructable",
    setup_setup_method_definitions_generator_methods_not_constructable,

    should_ok_when_use_generator_methods_not_constructable,
    r#"
      const obj = {
        g: function* () {
          let index = 0;
          while (true) {
            yield index++;
          }
        },
      };
    "#,
    1,

    should_ok_when_not_use_generator_methods_not_constructable,
    r#"
      const obj = {
        g: function () {
          let index = 0;
          while (true) {
            yield index++;
          }
        },
      };
    "#,
    0,
  }
}
