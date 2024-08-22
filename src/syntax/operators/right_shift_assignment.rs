use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./right_shift_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_assignment_expression.push(walk_assignment_expression);
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::ShiftRight)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_right_shift_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      let num = 16;
      num >>= 2;
      console.log(num);
    "#,
    1,
  }
}
