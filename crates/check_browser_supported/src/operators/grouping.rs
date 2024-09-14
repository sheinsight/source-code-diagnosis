use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  OperatorsGrouping,
  compat {
    name: "operators.grouping",
    description: "分组运算符 ()",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Grouping",
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
    matches!(node.kind(), AstKind::ParenthesizedExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsGrouping;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_grouping_operator:{
      setup: OperatorsGrouping::default(),
      source_code: r#"
        console.log((2 + 3) * 4);
      "#,
      eq: [
        r#"(2 + 3)"#,
      ],
      ne: []
    }
  }
}
