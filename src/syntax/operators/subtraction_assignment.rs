use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./subtraction_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_assignment_expression.push(walk_assignment_expression);
  },

  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Subtraction)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_subtraction_assignment",
    setup,
    should_ok_when_subtraction_assignment,
    r#"
      let x = 10;
      x -= 5;
    "#,
    1
  }
}
