use oxc_ast::ast::ArrowFunctionExpression;
use regex::Regex;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_arrow_function_expression.push(arrow_functions_trailing_comma);
  },
  compat {
    name: "arrow_functions_trailing_comma",
    description: "trailing commas in parameter lists of arrow functions",
    tags: [
      "web-features:arrow-functions-trailing-comma",
      "web-features:snapshot:ecmascript-2017"
    ],
    support: {
      chrome: "58",
      chrome_android: "58",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "8.0.0",
      deno: "1.0",
    }
  },
  arrow_functions_trailing_comma,
  |ctx: &mut Context, it: &ArrowFunctionExpression| {
    let params_source_code = ctx.get_source_code(it.params.span);
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      regex.is_match(params_source_code)
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
    "arrow_functions_trailing_comma",
    setup,

    should_ok_when_use_arrow_functions_trailing_comma,
    r#"
      const func4 = (
        a,
        b,
        c,
      ) => 1;
    "#,
    1,

    should_ok_when_not_use_arrow_functions_trailing_comma,
    r#"
      const func4 = (
        a,
        b,
        c
      ) => 1;
    "#,
    0,

    should_ok_when_use_arrow_functions_trailing_comma_with_no_space,
    r#"
      const func4 = (
        a,
        b,
        c,) => 1;
    "#,
    1,
  }
}
