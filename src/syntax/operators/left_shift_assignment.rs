use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_left_shift_assignment",
    description: "左移位赋值运算符 (<code>x &lt;&lt;= y</code>)",
    tags: ["web-features:snapshot:ecmascript-1"],
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
    matches!(it.operator, AssignmentOperator::ShiftLeft)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::setup;

  assert_ok_count! {
    "operators_left_shift_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let a = 5;
    a <<= 2;
    "#,
    1,
  }
}
