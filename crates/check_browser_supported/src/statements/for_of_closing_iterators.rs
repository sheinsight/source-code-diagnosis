use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  ForOfClosingIterators,
  compat {
    name: "statements.for_of.closing_iterators",
    description: "for...of 语句中的关闭迭代器",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for...of#Closing_iterators",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "51",
      chrome_android: "51",
      firefox: "53",
      firefox_android: "53",
      safari: "7",
      safari_ios: "7",
      edge: "14",
      node: "6.5.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    // TODO: 实现检测关闭迭代器的逻辑
    matches!(node.kind(), AstKind::ForOfStatement(_))
  }
}

#[cfg(test)]
mod tests {
  use super::ForOfClosingIterators;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_for_of_statement:{
      setup: ForOfClosingIterators::default(),
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
      setup: ForOfClosingIterators::default(),
      source_code: r#"
        const array1 = ['a', 'b', 'c'];
        array1.forEach(element => console.log(element));
      "#,
      eq: [],
      ne: [
        r#"array1.forEach(element => console.log(element));"#,
      ]
    }
  }
}
