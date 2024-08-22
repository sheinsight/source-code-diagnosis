use oxc_ast::ast::FunctionType;
use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  "./async_generator_function.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags, is_strict_mode: bool| {
    matches!(it.r#type, FunctionType::FunctionDeclaration) && it.r#async && it.generator
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "async_generator_function",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"async function* foo(){}"#,
    1,

    should_fail_when_async_function_declaration,
    r#"async function foo(){}"#,
    0,

    should_fail_when_generate_function_declaration,
    r#"function* foo(){}"#,
    0,

    should_fail_when_async_generate_function_expression,
    r#"const a = function* () {}"#,
    0,
  }
}
