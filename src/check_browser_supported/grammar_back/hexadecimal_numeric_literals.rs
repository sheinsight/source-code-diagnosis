use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_numeric_literal.push(walk_numeric_literal);
  },
  compat {
    name: "hexadecimal_numeric_literals",
    description: "十六进制数字字面量 (0xAF)",
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
  walk_numeric_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::NumericLiteral| {
    vec!["0x", "0X"].iter().any(|item| it.raw.starts_with(item))
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "hexadecimal_numeric_literals",
    setup,

    should_ok_when_use_hexadecimal_numeric_literals,
    r#"
      0xFFFFFFFFFFFFFFFFF; // 295147905179352830000
      0x123456789ABCDEF;   // 81985529216486900
      0XA;                 // 10
    "#,
    3,
  }
}
