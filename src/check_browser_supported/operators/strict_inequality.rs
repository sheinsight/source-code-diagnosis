use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_binary_expression.push(walk_binary_expression);
  },
  compat {
    name: "operators_strict_inequality",
    description: "Strict inequality (<code>a !== b</code>)",
    tags: [],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "4",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::StrictInequality)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_strict_inequality",
    setup,
    should_ok_when_strict_inequality,
    r#"
      console.log(5 !== 5);
    "#,
    1
  }
}
