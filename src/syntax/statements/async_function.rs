use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "async_function",
    description: "async function 语句",
    tags: [
      "web-features:async-await"
    ],
    support: {
      chrome: "55",
      chrome_android: "55",
      firefox: "52",
      firefox_android: "52",
      opera: "55",
      opera_android: "55",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "15",
      oculus: "55",
      node: "7.6.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags| {
    it.r#async && !it.generator
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "async_function",
    setup,

    should_ok_when_async_function_declaration,
    r#"async function hello(){}"#,
    1,

    should_fail_when_async_function_declaration,
    r#"function hello(){}"#,
    0,

    should_fail_when_async_generate_function_declaration,
    r#"function* hello(){}"#,
    0,

    should_fail_when_async_generate_function_expression,
    r#"const hello = async function* (){}"#,
    0,
  }
}
