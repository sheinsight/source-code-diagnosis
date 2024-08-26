use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_boolean_literal.push(walk_boolean_literal);
  },
  compat {
    name: "boolean_literals",
    description: "布尔字面量 (true/false)",
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
  walk_boolean_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::BooleanLiteral| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "boolean_literals",
    setup,

    should_ok_when_use_boolean_literals,
    r#"
      true;
      false;
    "#,
    2,
  }
}
