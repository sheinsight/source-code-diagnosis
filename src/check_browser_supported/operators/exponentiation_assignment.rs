use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_exponentiation_assignment",
    description: "幂赋值运算符 (<code>x **= y</code>)",
    tags: [
      "web-features:snapshot:ecmascript-2016"
    ],
    support: {
      chrome: "52",
      chrome_android: "52",
      firefox: "52",
      firefox_android: "52",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "14",
      node: "7.0.0",
      deno: "1.0",
    }
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Exponential)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_exponentiation_assignment",
    setup,
    should_ok_when_exponentiation_assignment,
    r#"
    let x = 2;
    x **= 3;
    "#,
    1,
  }
}
