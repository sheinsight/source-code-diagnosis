use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_yield_expression.push(walk_yield_expression);
  },
  compat {
    name: "operators_yield_star",
    description: "yield*",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "39",
      chrome_android: "39",
      firefox: "27",
      firefox_android: "27",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_yield_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::YieldExpression| {
    it.delegate
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_yield_star",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
  function* generatorB() {
    yield 'Generator B:';
    yield* generatorA();
    yield 'End of B';
  }
    "#,
    1
  }
}
