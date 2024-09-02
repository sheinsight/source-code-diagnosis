use oxc_ast::ast::FunctionType;
use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "generator_function_trailing_comma_in_parameters",
    description: "生成器函数参数中的尾随逗号",
    tags: [
      "web-features:snapshot:ecmascript-2015"
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
    !it.r#async &&
    it.generator &&
    ctx.source_code[it.params.span.start as usize..it.params.span.end as usize].ends_with(",)")
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count!(
    "generator_function_trailing_comma_in_parameters",
    setup,
    should_ok_when_generator_function_declaration,
    r#"
    function* calcRectArea(width, height,) {
      return width * height;
    }
    "#,
    1,
    should_ok_when_generator_function_expression,
    r#"
    const calcRectArea = function* (width, height,) {
      return width * height;
    }
    "#,
    0,
  );
}
