use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_multiplication_assignment",
    description: "乘法赋值运算符 (<code>x *= y</code>)",
    tags: ["web-features:snapshot:ecmascript-1"],
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
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Multiplication)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
                "operators_multiplication_assignment",
                setup,

                should_ok_when_multiplication_assignment,
                r"
let a = 2;
a *= 3;
console.log(a);
",
                1,
              }
}
