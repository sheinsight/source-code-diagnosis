use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./logical_or_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
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
