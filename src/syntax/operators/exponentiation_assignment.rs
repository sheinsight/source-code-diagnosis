use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./exponentiation_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
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
