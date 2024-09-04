use crate::create_compat_2;

create_compat_2! {
  NullLiteral,
  compat {
    name: "null_literal",
    description: "Null literal",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/null",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "1.0.0",
      firefox: "1.0.0",
      firefox_android: "1.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::NullLiteral(_))
  }
}

#[cfg(test)]
mod tests {
  use super::NullLiteral;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_null_literal:{
      setup: NullLiteral::default(),
      source_code: r#"
        let x = null;
        console.log(null);
      "#,
      eq: [
        r#"null"#,
        r#"null"#,
      ],
      ne: []
    },

    should_fail_when_not_null_literal:{
      setup: NullLiteral::default(),
      source_code: r#"
        let x = undefined;
        console.log(nul);
      "#,
      eq: [],
      ne: [
        r#"undefined"#,
        r#"nul"#,
      ]
    }
  }
}
