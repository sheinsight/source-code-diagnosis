use oxc_ast::ast::FunctionType;
use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "generator_function",
    description: "function* 语句",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "39",
      chrome_android: "39",
      firefox: "26",
      firefox_android: "26",
      opera: "39",
      opera_android: "39",
      safari: "10",
      safari_ios: "10",
      edge: "13",
      oculus: "39",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags| {
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
