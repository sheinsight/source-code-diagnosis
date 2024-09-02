use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_null_literal.push(walk_null_literal);
  },
  compat {
    name: "operators_null",
    description: "The value null represents the intentional absence of any object value.",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_null_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::NullLiteral| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_null",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    let obj = null;
    "#,
    1,
  }
}
