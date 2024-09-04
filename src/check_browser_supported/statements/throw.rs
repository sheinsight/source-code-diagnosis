use crate::create_compat_2;

create_compat_2! {
  Throw,
  compat {
    name: "statements.throw",
    description: "throw 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/throw",
    tags: [
      "web-features:snapshot:ecmascript-3"
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
    matches!(node.kind(), AstKind::ThrowStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::Throw;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_throw_statement:{
      setup: Throw::default(),
      source_code: r#"
        throw new Error('Parameter is not a number!');
      "#,
      eq: [
        r#"throw new Error('Parameter is not a number!');"#,
      ],
      ne: []
    }
  }
}
