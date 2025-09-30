use crate::create_compat;

create_compat! {
  TemplateLiteralsTemplateLiteralRevision,
  compat {
    name: "template_literals.template_literal_revision",
    description: "标签模板字面量中允许的转义序列",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals#tagged_templates_and_escape_sequences",
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
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, _node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    // matches!(node.kind(), AstKind::TaggedTemplateExpression(_))
    // TODO
    false
  }
}

#[cfg(test)]
mod tests {
  use super::TemplateLiteralsTemplateLiteralRevision;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_tagged_template_literals: {
      setup: TemplateLiteralsTemplateLiteralRevision::default(),
      source_code: r#"
        function tag(strings, ...values) {
          return strings.raw[0];
        }

        tag`\unicode`;
      "#,
      eq: [
        // r#"tag`\unicode`"#,
      ],
      ne: []
    },

    should_fail_when_not_tagged_template_literals: {
      setup: TemplateLiteralsTemplateLiteralRevision::default(),
      source_code: r#"
        `\unicode`;
      "#,
      eq: [],
      ne: [
        r#"`\unicode`"#,
      ]
    }
  }
}
