use crate::create_compat_2;

create_compat_2! {
  StringLiterals,
  compat {
    name: "string_literals",
    description: "String literals ('Hello world')",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String",
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
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::StringLiteral(_))
  }
}

#[cfg(test)]
mod tests {
  use super::StringLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_string_literals:{
      setup: StringLiterals::default(),
      source_code: r#"
        'foo';
        "bar";
      "#,
      eq: [
        r#"'foo'"#,
        r#""bar""#,
      ],
      ne: []
    }
  }
}
