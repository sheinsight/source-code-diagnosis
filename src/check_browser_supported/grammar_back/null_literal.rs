use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_null_literal.push(walk_null_literal);
  },
  compat {
    name: "null_literal",
    description: "空值字面量 (null)",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
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
    "null_literal",
    setup,

    should_ok_when_use_null_literal,
    r#"
      null;
    "#,
    1,

    should_fail_when_null_literal,
    r#"
      nul;
    "#,
    0,
  }
}
