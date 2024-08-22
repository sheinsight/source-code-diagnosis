use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  "./generator_function.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
  },

  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags, is_strict_mode: bool| {
    it.generator && !it.r#async
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_generator_function",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    const foo = function* () {
      yield 'a';
      yield 'b';
      yield 'c';
    };
    "#,
    1,
  }
}
