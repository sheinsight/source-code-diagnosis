use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_numeric_literal.push(walk_numeric_literal);
  },
  compat {
    name: "binary_numeric_literals",
    description: "二进制数字字面量 (0b)",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "41",
      chrome_android: "41",
      firefox: "25",
      firefox_android: "25",
      opera: "41",
      opera_android: "41",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      oculus: "41",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_numeric_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::NumericLiteral| {
    vec!["0b", "0B"]
      .iter()
      .any(|item| it.raw.starts_with(*item))
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "binary_numeric_literals",
    setup,

    should_ok_when_use_binary_numeric_literals,
    r#"
      0b10000000000000000000000000000000 // 2147483648
      0b01111111100000000000000000000000 // 2139095040
      0B00000000011111111111111111111111 // 8388607
    "#,
    3,
  }
}
