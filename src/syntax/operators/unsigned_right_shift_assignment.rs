use oxc_syntax::operator::AssignmentOperator;

crate::create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_unsigned_right_shift_assignment",
    description: "Unsigned right shift assignment (<code>x >>>= y</code>)",
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
    matches!(it.operator, AssignmentOperator::ShiftRightZeroFill)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_unsigned_right_shift_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
let a = 8;
a >>>= 2;
"#,
    1,
  }
}
