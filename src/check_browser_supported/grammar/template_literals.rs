use crate::create_compat_2;

create_compat_2! {
  TemplateLiterals,
  compat {
    name: "template_literals",
    description: "Template literals",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Template_literals",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "41.0.0",
      chrome_android: "41.0.0",
      firefox: "34.0.0",
      firefox_android: "34.0.0",
      safari: "9.0.0",
      safari_ios: "9.0.0",
      edge: "12.0.0",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::TemplateLiteral(_))
  }
}

#[cfg(test)]
mod tests {
  use super::TemplateLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_template_literals:{
      setup: TemplateLiterals::default(),
      source_code: r#"
        `foo`;
        `bar`;
        `baz ${1 + 2}`;
      "#,
      eq: [
        r#"`foo`"#,
        r#"`bar`"#,
        r#"`baz ${1 + 2}`"#,
      ],
      ne: []
    }
  }
}
