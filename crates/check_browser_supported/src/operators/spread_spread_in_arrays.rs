use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  SpreadInArrays,
  compat {
    name: "operators.spread.spread_in_arrays",
    description: "数组字面量中的展开语法",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax#spread_in_array_literals",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "16",
      firefox_android: "16",
      safari: "8",
      safari_ios: "8",
      edge: "12",
      node: "5.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, nodes: &AstNodes<'a>) -> bool {
    if let AstKind::SpreadElement(_) = node.kind() {
      if let Some(parent_node) = nodes.parent_node(node.id()) {
        return matches!(parent_node.kind(), AstKind::ArrayExpressionElement(_))
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::SpreadInArrays;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_detect_spread_in_array_literals:{
      setup: SpreadInArrays::default(),
      source_code: r#"
        const parts = ["shoulders", "knees"];
        const lyrics = ["head", ...parts, "and", "toes"];
      "#,
      eq: [
        r#"...parts"#,
      ],
      ne: []
    },

    should_not_detect_spread_in_function_calls:{
      setup: SpreadInArrays::default(),
      source_code: r#"
        function myFunction(x, y, z) {}
        const args = [0, 1, 2];
        myFunction(...args);
      "#,
      eq: [],
      ne: [
        r#"...args"#,
      ]
    }
  }
}
