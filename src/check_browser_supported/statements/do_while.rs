use crate::create_compat_2;

create_compat_2! {
  DoWhileStatement,
  compat {
    name: "statements.do_while",
    description: "do...while 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/do...while",
    tags: ["web-features:snapshot:ecmascript-3"],
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
    matches!(node.kind(), AstKind::DoWhileStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::DoWhileStatement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_do_while_statement:{
      setup: DoWhileStatement::default(),
      source_code: r#"
        let i = 0;
        let result = 0;
        do {
          i = i + 1;
          result = result + i;
        } while (i < 5);
      "#,
      eq: [
        r#"do {
          i = i + 1;
          result = result + i;
        } while (i < 5);"#,
      ],
      ne: []
    }
  }
}
