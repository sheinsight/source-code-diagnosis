use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./addition_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::Addition)
  }
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::operators::addition_assignment::setup};

  assert_ok_count! {
    "operators_addition_assignment",
    setup,

    should_ok_when_use_addition_assignment,
    r#"
      let a = 2;
      let b = 'hello';
      console.log((a += 3));
    "#,
    1,

    should_ok_when_use_addition_assignment_with_string,
    r#"
      let a = 'hello';
      let b = 'world';
      console.log((a += b));
    "#,
    1,
  }
}
