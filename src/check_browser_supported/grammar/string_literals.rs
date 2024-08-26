use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_string_literal.push(walk_string_literal);
  },
  compat {
    name: "string_literals",
    description: "字符串字面量 ('Hello world')",
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
  walk_string_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::StringLiteral| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "string_literals",
    setup,

    should_ok_when_use_string_literals,
    r#"
      'foo';
      "bar";
    "#,
    2,
  }
}
