use crate::create_compat_2;

create_compat_2! {
  ForIn,
  compat {
    name: "statements.for_in",
    description: "for...in 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for...in",
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
    matches!(node.kind(), AstKind::ForInStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::ForIn;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_for_in_statement:{
      setup: ForIn::default(),
      source_code: r#"
        const object = { a: 1, b: 2, c: 3 };
        for (const property in object) { }
      "#,
      eq: [
        r#"for (const property in object) { }"#,
      ],
      ne: []
    },
    should_ng_when_not_use_for_in_statement:{
      setup: ForIn::default(),
      source_code: r#"
        const array = [1, 2, 3];
        for (let i = 0; i < array.length; i++) { }
      "#,
      eq: [],
      ne: [
        r#"for (let i = 0; i < array.length; i++) { }"#,
      ]
    }
  }
}
