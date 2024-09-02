use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_unary_expression.push(walk_unary_expression);
  },
  compat {
    name: "operators_logical_not",
    description: "逻辑非运算符 (<code>!</code>)",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::LogicalNot)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::setup;

  assert_ok_count! {
    "operators_logical_not",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(!true);
    "#,
    1,
  }
}
