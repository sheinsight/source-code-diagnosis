use oxc_ast::ast::FunctionType;
use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "function_trailing_comma_in_parameters",
    description: "Trailing comma in parameters",
    tags: [
      "web-features:snapshot:ecmascript-2017"
    ],
    support: {
      chrome: "58",
      chrome_android: "58",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      node: "8.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags| {
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
