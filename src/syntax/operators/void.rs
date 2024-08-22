use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  "./void.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_unary_expression.push(walk_unary_expression);
  },

  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::Void)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_void",
    setup,
    should_ok_when_console_log,
    r#"
console.log(void 0);
"#,
    1,
  }
}
