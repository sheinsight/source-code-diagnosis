use oxc_syntax::operator::UnaryOperator;

use crate::create_compat;

create_compat! {
  "./unary_plus.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_unary_expression.push(walk_unary_expression);
  },

  walk_unary_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::UnaryExpression| {
    matches!(it.operator, UnaryOperator::UnaryPlus)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_unary_plus",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
let x = 5;
console.log(+x);    
"#,
    1,
  }
}
