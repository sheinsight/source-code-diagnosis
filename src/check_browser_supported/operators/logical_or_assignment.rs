use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  compat {
    name: "operators_logical_or_assignment",
    description: "逻辑或赋值运算符 (<code>x ||= y</code>)",
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
    matches!(it.operator, AssignmentOperator::LogicalOr)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::setup;

  assert_ok_count! {
    "operators_logical_or_assignment",
    setup,

    should_ok_when_logical_or_assignment,
    r#"
    let a = null;
    let b = 3;

    a ||= 5;
    "#,1
  }
}
