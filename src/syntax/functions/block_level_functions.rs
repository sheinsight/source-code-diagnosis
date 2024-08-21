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
  flags: &oxc_semantic::ScopeFlags,
  is_strict_mode: bool,
) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./block_level_functions.json")).unwrap()
  });

  if is_strict_mode {
    if let Some(parent) = ctx.stack.last() {
      if matches!(parent, AstKind::BlockStatement(_)) {
        ctx
          .usage
          .push(CompatBox::new(it.span.clone(), compat.clone()));
      }
    }
  }
}

pub fn setup_block_level_functions(v: &mut SyntaxVisitor) {
  v.walk_function.push(walk_function);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::functions::block_level_functions::setup_block_level_functions,
  };

  assert_ok_count! {
    "functions_block_level_functions",
    setup_block_level_functions,

    should_ok_when_use_block_level_functions,
    r#"
      "use strict";
      {
        function f() {
          return 2;
        }
      }
    "#,
    1,

    should_ok_when_not_use_block_level_functions,
    r#"
      function f() {
        return 2;
      }
    "#,
    0,

  }
}
