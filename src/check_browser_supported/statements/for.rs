use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  For,
  compat {
    name: "statements.for",
    description: "for 语句创建一个由三个可选表达式组成的循环，这些表达式用括号括起来，用分号分隔，后跟一个在循环中执行的语句（通常是块语句）。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for",
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
    matches!(node.kind(), AstKind::ForStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::For;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_for_statement:{
      setup: For::default(),
      source_code: r#"
        for (let i = 0; i < 9; i++) {
          console.log(i);
        }
      "#,
      eq: [
        r#"for (let i = 0; i < 9; i++) {
          console.log(i);
        }"#,
      ],
      ne: []
    }
  }
}
