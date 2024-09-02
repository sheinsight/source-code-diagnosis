use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_logical_and_assignment",
    description: "逻辑与赋值运算符 (<code>x &&= y</code>)",
    tags: [],
    support: {
      chrome: "85",
      chrome_android: "85",
      firefox: "79",
      firefox_android: "79",
      safari: "14",
      safari_ios: "14",
      edge: "85",
      node: "15.0.0",
      deno: "1.2",
    }
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::LogicalAnd)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_logical_and_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let a = 1;
    let b = 0;
    a &&= 2;
    "#,
    1,
  }
}
