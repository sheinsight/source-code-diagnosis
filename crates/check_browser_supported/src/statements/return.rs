use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  Return,
  compat {
    name: "statements.return",
    description: "return 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/return",
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
    matches!(node.kind(), AstKind::ReturnStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::Return;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_return_statement:{
      setup: Return::default(),
      source_code: r#"
        function getRectArea(width, height) {
          if (width > 0 && height > 0) {
            return width * height;
          }
          return 0;
        }
      "#,
      eq: [
        r#"return width * height;"#,
        r#"return 0;"#
      ],
      ne: []
    }
  }
}
