use oxc_ast::ast::Function;
use oxc_semantic::ScopeFlags;
use regex::Regex;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "generator_function_trailing_comma",
    description: "Trailing comma in parameters",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "58",
      chrome_android: "58",
      firefox: "52",
      firefox_android: "52",
      opera: "58",
      opera_android: "58",
      safari: "10",
      safari_ios: "10",
      edge: "58",
      oculus: "58",
      node: "8.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &Function, _flags: &ScopeFlags| {
    if it.generator && !it.r#async {
      let source_code = &ctx.source_code[it.params.span.start as usize..it.params.span.end as usize];
      if let Ok(regex) = Regex::new(r",\s*\)$") {
        regex.is_match(source_code)
      } else {
        false
      }
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
    "generator_function_trailing_comma",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    const foo = function* (a,b,) {
      yield 'a';
      yield 'b';
      yield 'c';
    };
    "#,
    1,
  }
}
