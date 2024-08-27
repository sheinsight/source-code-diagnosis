use oxc_ast::ast::UnaryExpression;
use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_unary_expression.push(walk_unary_expression);
  },
  compat {
    name: "operators_bitwise_not",
    description: "按位非运算符 (<code>~a</code>)",
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
  |ctx: &mut Context, it: &UnaryExpression| {
    matches!(it.operator, UnaryOperator::BitwiseNot)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_bitwise_not",
    setup,

    should_ok_when_use_bitwise_not,
    r#"
      console.log(~5);
    "#,
    1,

    should_ok_when_use_bitwise_not_with_parentheses,
    r#"
      console.log(~(5));
    "#,
    1
  }
}
