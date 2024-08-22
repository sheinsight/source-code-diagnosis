use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./bitwise_xor_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },

  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::BitwiseXOR)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_bitwise_xor_assignment",
    setup,

    should_ok_when_use_bitwise_xor_assignment,
    r#"
      let a = 5;
      a ^= 3;
      console.log(a);
    "#,
    1
  }
}
