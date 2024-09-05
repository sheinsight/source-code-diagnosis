use crate::create_compat_2;

create_compat_2! {
  BooleanLiterals,
  compat {
    name: "boolean_literals",
    description: "Boolean literals (<code>true</code>/<code>false</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Boolean_literal",
    tags: [
      "web-features:array",
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
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::BooleanLiteral(_))
  }
}

#[cfg(test)]
mod tests {
  use super::BooleanLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_array_literals:{
      setup: BooleanLiterals::default(),
      source_code: r#"
        true;
        false;
      "#,
      eq: [
        r#"true"#,
        r#"false"#,
      ],
      ne: []
    }
  }
}
