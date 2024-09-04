use crate::create_compat_2;

create_compat_2! {
  TemplateLiteralsTemplateLiteralRevision,
  compat {
    name: "template_literals_template_literal_revision",
    description: "Escape sequences allowed in tagged template literals",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#tagged_templates_and_escape_sequences",
    tags: [
      "web-features:snapshot:ecmascript-2018"
    ],
    support: {
      chrome: "62.0.0",
      chrome_android: "62.0.0",
      firefox: "53.0.0",
      firefox_android: "53.0.0",
      safari: "11.0.0",
      safari_ios: "11.0.0",
      edge: "62.0.0",
      node: "8.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::TemplateLiteral(_))
  }
}

#[cfg(test)]
mod tests {
  use super::TemplateLiteralsTemplateLiteralRevision;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_template_literals_template_literal_revision: {
      setup: TemplateLiteralsTemplateLiteralRevision::default(),
      source_code: r#"
        `foo`;
        `bar`;
        tag`Hello ${ a + b } world ${ a * b }`;
      "#,
      eq: [
        r#"`foo`"#,
        r#"`bar`"#,
        r#"`Hello ${ a + b } world ${ a * b }`"#,
      ],
      ne: []
    }
  }
}
