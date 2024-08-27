use oxc_ast::ast::Argument;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_call_expression.push(walk_call_expression);
  },
  compat {
    name: "spread_in_function_calls",
    description: "Spread in function calls",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "27",
      firefox_android: "27",
      safari: "8",
      safari_ios: "8",
      edge: "12",
      node: "5.0.0",
      deno: "1.0",
    }
  },
  walk_call_expression,
  |ctx: &mut Context, expr: &oxc_ast::ast::CallExpression| {
    expr.arguments.iter().any(|arg| matches!(arg, Argument::SpreadElement(_)))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "spread_in_function_calls",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      function myFunction(x, y, z) {}
      const args = [0, 1, 2];
      myFunction(...args);     
    "#,
    1
  }
}
