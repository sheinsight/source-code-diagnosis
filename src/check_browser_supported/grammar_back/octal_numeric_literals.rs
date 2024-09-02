use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_numeric_literal.push(walk_numeric_literal);
  },
  compat {
    name: "octal_numeric_literals",
    description: "八进制数字字面量 (0o)",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "41",
      chrome_android: "41",
      firefox: "25",
      firefox_android: "25",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_numeric_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::NumericLiteral| {
    vec!["0O", "0o"].iter().any(|item| it.raw.starts_with(item))
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "octal_numeric_literals",
    setup,

    should_ok_when_use_octal_numeric_literals,
    r#"
      0O755;
      0o644;
    "#,
    2,
  }
}
