use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  EmptyStatement,
  compat {
    name: "statements.empty",
    description: "空语句 (;)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/Empty",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "3",
      chrome_android: "3",
      firefox: "1",
      firefox_android: "1",
      safari: "5",
      safari_ios: "5",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::EmptyStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::EmptyStatement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_empty_statement:{
      setup: EmptyStatement::default(),
      source_code: r#"
        ;
        let a = 1;;
        if (true);
      "#,
      eq: [
        r#";"#,
        r#";"#,
        r#";"#,
      ],
      ne: []
    }
  }
}
