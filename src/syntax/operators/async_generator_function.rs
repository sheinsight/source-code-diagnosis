use crate::create_compat;

create_compat! {
  "./async_generator_function.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
  },

  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function,flags: &oxc_semantic::ScopeFlags,is_strict_mode: bool| {
    it.is_expression() && it.r#async && it.generator
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count, syntax::operators::async_generator_function::setup,
  };

  assert_ok_count! {
    "operators_async_generator_function",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
      const asyncGenerator = async function* () {
        yield await Promise.resolve(1);
        yield await Promise.resolve(2);
        yield await Promise.resolve(3);
      };
    "#,
    1,

  }
}
