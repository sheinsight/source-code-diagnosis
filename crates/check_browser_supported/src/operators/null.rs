use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  NullLiteral,
  compat {
    name: "operators.null",
    description: "Null 字面量",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/null",
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
    }
  }
}
