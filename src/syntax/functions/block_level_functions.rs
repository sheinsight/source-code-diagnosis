use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(block_level_functions);
  },
  compat {
    name: "block_level_functions",
    description: "block-level function declarations in strict mode",
    tags: [
      "web-features:block-level-functions",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "46",
      firefox_android: "46",
      opera: "36",
      opera_android: "36",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      oculus: "5.0",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  block_level_functions,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &oxc_semantic::ScopeFlags, is_strict_mode: bool| {
    if is_strict_mode {
      if let Some(parent) = ctx.stack.last() {
        matches!(parent, AstKind::BlockStatement(_))
      } else {
        false
      }
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "block_level_functions",
    setup,

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
