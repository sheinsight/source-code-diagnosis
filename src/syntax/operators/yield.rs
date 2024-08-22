use crate::create_compat;

create_compat! {
  "./yield.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_yield_expression.push(walk_yield_expression);
  },

  walk_yield_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::YieldExpression| {
    !it.delegate
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_yield",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
  function* countAppleSales() {
    yield 3;
    yield 7;
    yield* 5;
  }
  "#,
  2
  }
}
