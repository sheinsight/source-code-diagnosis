use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_yield_expression.push(walk_yield_expression);
  },
  compat {
    name: "operators_yield",
    description: "yield operator",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "39",
      chrome_android: "39",
      firefox: "26",
      firefox_android: "26",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_yield_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::YieldExpression| {
    !it.delegate
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_yield",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
  function* countAppleSales() {
    yield 3;
    yield 7;
    yield* 5;
  }
  "#,
  2
  }
}
