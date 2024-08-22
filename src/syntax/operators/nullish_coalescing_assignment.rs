use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat;

create_compat! {
  "./nullish_coalescing_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
  },

  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::LogicalNullish)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_nullish_coalescing_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let x = null;
    let y = 5;
    x ??= y;
    console.log(x);
    "#,
    1,
  }
}
