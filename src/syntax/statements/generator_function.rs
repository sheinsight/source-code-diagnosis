use oxc_ast::ast::FunctionType;
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
    matches!(it.r#type, FunctionType::FunctionDeclaration) && !it.r#async && it.generator
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count!(
    "generator_function",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    function* generator(i) {
      yield i;
      yield i + 10;
    }
    "#,
    1,
    should_fail_when_async_generator_function_expression,
    r#"
    const generator = function* (i) {
      yield i;
      yield i + 10;
    }
    "#,
    0,
  );
}
