use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_unary_expression.push(walk_unary_expression);
  },
  compat {
    name: "operators_void",
    description: "The void operator evaluates the given expression and then returns undefined.",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "3.1",
      safari_ios: "3",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::Void)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_void",
    setup,
    should_ok_when_console_log,
    r#"
console.log(void 0);
"#,
    1,
  }
}
