use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_template_literal.push(walk_template_literal);
  },
  compat {
    name: "template_literals",
    description: "模板字面量",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "41",
      chrome_android: "41",
      firefox: "34",
      firefox_android: "34",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_template_literal,
  |ctx: &mut Context, it: &oxc_ast::ast::TemplateLiteral| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "template_literals",
    setup,

    should_ok_when_use_template_literals,
    r#"
      `foo`;
      `bar`;
    "#,
    2,
  }
}
