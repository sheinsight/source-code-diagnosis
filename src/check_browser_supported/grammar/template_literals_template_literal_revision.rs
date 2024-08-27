use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_template_literal.push(walk_template_literal);
  },
  compat {
    name: "template_literals_template_literal_revision",
    description: "标签模板字面量中允许的转义序列",
    tags: [
      "web-features:snapshot:ecmascript-2018"
    ],
    support: {
      chrome: "62",
      chrome_android: "62",
      firefox: "53",
      firefox_android: "53",
      safari: "11",
      safari_ios: "11",
      edge: "62",
      node: "8.10.0",
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
    "template_literals_template_literal_revision",
    setup,

    should_ok_when_use_template_literals_template_literal_revision,
    r#"
      `foo`;
      `bar`;
    "#,
    2,
  }
}
