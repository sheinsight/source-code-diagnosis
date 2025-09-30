use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  While,
  compat {
    name: "statements.while",
    description: "while 循环",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/while",
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
