use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./left_shift_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
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
