use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  IfElse,
  compat {
    name: "statements.if_else",
    description: "if...else 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/if...else",
    tags: ["web-features:snapshot:ecmascript-1"],
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
