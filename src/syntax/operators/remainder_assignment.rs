use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./remainder_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_assignment_expression.push(walk_assignment_expression);
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Remainder)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_remainder_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      let number = 10;
      number %= 3;
    "#,
    1,
  }
}
