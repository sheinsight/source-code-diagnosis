use oxc_syntax::operator::LogicalOperator;

use crate::create_compat;

create_compat! {
  "./logical_or.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_logical_expression.push(walk_logical_expression);
  },

  walk_logical_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::LogicalExpression| {
    matches!(it.operator, LogicalOperator::Or)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_logical_or",
    setup,

    should_ok_when_logical_or,
    r"console.log(true || true);",
    1,
  }
}
