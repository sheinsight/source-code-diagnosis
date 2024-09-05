use crate::create_compat_2;

create_compat_2! {
  LabelStatement,
  compat {
    name: "statements.label",
    description: "标签语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/label",
    tags: [
      "web-features:snapshot:ecmascript-3"
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
    matches!(node.kind(), AstKind::LabeledStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::LabelStatement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_label_statement:{
      setup: LabelStatement::default(),
      source_code: r#"
        let str = '';

        loop1: for (let i = 0; i < 5; i++) {
          if (i === 1) {
            continue loop1;
          }
          str = str + i;
        }
      "#,
      eq: [
        r#"loop1: for (let i = 0; i < 5; i++) {
          if (i === 1) {
            continue loop1;
          }
          str = str + i;
        }"#,
      ],
      ne: []
    }
  }
}
