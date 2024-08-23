use oxc_syntax::operator::LogicalOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_logical_expression.push(walk_logical_expression);
  },
  compat {
    name: "operators_logical_and",
    description: "逻辑与运算符 (<code>&amp;&amp;</code>)",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
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
  walk_logical_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::LogicalExpression| {
    matches!(it.operator, LogicalOperator::And)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_logical_and",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    console.log(true && true);
    "#,
    1,
  }
}
