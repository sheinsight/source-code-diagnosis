use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
  },
  compat {
    name: "operators_function",
    description: "<code>function</code> 表达式",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags, is_strict_mode: bool| {
    it.is_expression()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_function",
    setup,

    should_ok_when_function_expression,
    r#"
    let greet = function(name) {
        console.log("Hello, " + name);
    };
    greet("Alice");
    "#,
    1,

    should_ok_when_iife,
    r#"
    (function() {
        console.log("This function is executed immediately.");
    })();
    "#,
    1,
  }
}
