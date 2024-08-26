use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_bitwise_xor_assignment",
    description: "按位异或赋值运算符 (<code>x ^= y</code>)",
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
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::BitwiseXOR)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_bitwise_xor_assignment",
    setup,

    should_ok_when_use_bitwise_xor_assignment,
    r#"
      let a = 5;
      a ^= 3;
      console.log(a);
    "#,
    1
  }
}
