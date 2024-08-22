use oxc_semantic::ScopeFlags;
use regex::Regex;

use crate::create_compat;

create_compat! {
  "./generator_function_trailing_comma.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
  },

  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags, is_strict_mode: bool| {
    if it.generator && !it.r#async {
      let source_code = &ctx.source_code[it.params.span.start as usize..it.params.span.end as usize];
      if let Ok(regex) = Regex::new(r",\s*\)$") {
        regex.is_match(source_code)
      }else {
        false
      }
    }else {
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
