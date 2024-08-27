use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
  },
  compat {
    name: "operators_generator_function",
    description: "<code>function*</code> 表达式",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "26",
      firefox_android: "26",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags| {
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
