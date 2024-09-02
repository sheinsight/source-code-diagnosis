use oxc_semantic::ScopeFlags;
use regex::Regex;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "operators_function_trailing_comma",
    description: "函数参数中的尾随逗号",
    tags: [],
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
  |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &ScopeFlags| {
    let source_code = &ctx.source_code[it.params.span.start as usize..it.params.span.end as usize];
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      regex.is_match(source_code)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_function_trailing_comma",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let greet = function(name,age,) {
        console.log("Hello, " + name);
    };
    greet("Alice");
    "#,
    1,
  }
}
