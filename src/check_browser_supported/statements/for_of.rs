use crate::create_compat_2;

create_compat_2! {
  ForOf,
  compat {
    name: "statements.for_of",
    description: "for...of 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/for...of",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "38.0.0",
      chrome_android: "38.0.0",
      firefox: "13.0.0",
      firefox_android: "13.0.0",
      safari: "7.0.0",
      safari_ios: "7.0.0",
      edge: "12.0.0",
      node: "0.12.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ForOfStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::ForOf;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_for_of_statement:{
      setup: ForOf::default(),
      source_code: r#"
        const array1 = ['a', 'b', 'c'];
        for (const element of array1) {
          console.log(element);
        }
      "#,
      eq: [
        r#"for (const element of array1) {
          console.log(element);
        }"#,
      ],
      ne: []
    },
    should_ng_when_not_use_for_of_statement:{
      setup: ForOf::default(),
      source_code: r#"
        const array1 = ['a', 'b', 'c'];
        for (let i = 0; i < array1.length; i++) {
          console.log(array1[i]);
        }
      "#,
      eq: [],
      ne: [
        r#"for (let i = 0; i < array1.length; i++) {
          console.log(array1[i]);
        }"#,
      ]
    }
  }
}
