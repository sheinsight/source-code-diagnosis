use crate::create_compat;

create_compat! {
  "./async_function.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
      v.walk_arrow_function_expression.push(walk_arrow_function_expression);
  },

  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function,flags: &oxc_semantic::ScopeFlags,is_strict_mode: bool| {
    it.is_expression() && it.r#async && !it.generator
  },

  walk_arrow_function_expression,
  |ctx: &mut Context,it: &oxc_ast::ast::ArrowFunctionExpression| {
    it.r#async
  }

}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::operators::async_function::setup};

  assert_ok_count! {
    "operators_async_function",
    setup,

    should_ok_when_async_function_declaration,
    r#"
      const asyncFunction = async function() {
      };
    "#,
    1,

    should_ok_when_async_function_declaration_with_arrow,
    r#"
      const asyncFunction = async () => {
      };
    "#,
    1,
  }
}
