use crate::create_compat_2;

create_compat_2! {
  While,
  compat {
    name: "while",
    description: "while 循环",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/while",
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
    matches!(node.kind(), AstKind::WhileStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::While;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_while_statement:{
      setup: While::default(),
      source_code: r#"
        let n = 0;
        while (n < 3) {
          n++;
        }
      "#,
      eq: [
        r#"while (n < 3) {
          n++;
        }"#,
      ],
      ne: []
    }
  }
}
