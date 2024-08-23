use oxc_ast::ast::FunctionType;
use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  "./function_trailing_comma_in_parameters.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },

  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags, is_strict_mode: bool| {
    matches!(it.r#type, FunctionType::FunctionDeclaration) &&
    ctx.source_code[it.params.span.start as usize..it.params.span.end as usize].ends_with(",)")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "function_trailing_comma_in_parameters",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    function calcRectArea(width, height,) {
      return width * height;
    }
    "#,
    1
  }
}
