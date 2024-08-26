use oxc_syntax::operator::BinaryOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_binary_expression.push(walk_binary_expression);
  },
  compat {
    name: "operators_exponentiation",
    description: "幂运算符 (<code>**</code>)",
    tags: ["web-features:snapshot:ecmascript-2016"],
    support: {
      chrome: "52",
      chrome_android: "52",
      firefox: "52",
      firefox_android: "52",
      opera: "52",
      opera_android: "52",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "14",
      oculus: "52",
      node: "7.0.0",
      deno: "1.0",
    }
  },
  walk_binary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::BinaryExpression| {
    matches!(it.operator, BinaryOperator::Exponential)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_exponentiation",
    setup,

    should_ok_when_exponentiation,
    r#"
    console.log(2 ** 3);
    "#,
    1,
  }
}
