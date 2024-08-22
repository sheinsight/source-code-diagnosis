use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./division_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },

  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Division)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "division_assignment",
    setup,

    should_ok_when_division_assignment,
    r#"
      let a = 10;
      a /= 2;
      console.log(a);
    "#,
    1,
  }
}
