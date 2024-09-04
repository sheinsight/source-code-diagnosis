use crate::create_compat_2;

create_compat_2! {
  IfElse,
  compat {
    name: "if_else",
    description: "if...else 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/if...else",
    tags: ["web-features:snapshot:ecmascript-1"],
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
    matches!(node.kind(), AstKind::IfStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::IfElse;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_if_else_statement:{
      setup: IfElse::default(),
      source_code: r#"
        function testNum(a) {
          let result;
          if (a > 0) {
            result = 'positive';
          } else {
            result = 'NOT positive';
          }
          return result;
        }
      "#,
      eq: [
        r#"if (a > 0) {
            result = 'positive';
          } else {
            result = 'NOT positive';
          }"#,
      ],
      ne: []
    }
  }
}
