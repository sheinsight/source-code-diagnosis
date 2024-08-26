use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_numeric_literal.push(walk_numeric_literal);
  },
  compat {
    name: "decimal_numeric_literals",
    description: "十进制数字字面量 (1234567890)",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_numeric_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::NumericLiteral| {
    !it.raw.starts_with("0")
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "decimal_numeric_literals",
    setup,

    should_ok_when_use_decimal_numeric_literals,
    r#"
      1234567890;
      42;
    "#,
    2,

    should_fail_when_decimal_numeric_literals,
    r#"
      0B42;
    "#,
    0,
  }
}
