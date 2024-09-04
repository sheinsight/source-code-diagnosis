use crate::create_compat_2;

create_compat_2! {
  SpreadInObjectLiterals,
  compat {
    name: "operators.spread.spread_in_object_literals",
    description: "对象字面量中的展开语法",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax#spread_in_object_literals",
    tags: ["web-features:snapshot:ecmascript-2018"],
    support: {
      chrome: "60.0.0",
      chrome_android: "60.0.0",
      firefox: "55.0.0",
      firefox_android: "55.0.0",
      safari: "11.1.0",
      safari_ios: "11.1.0",
      edge: "60.0.0",
      node: "8.3.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, nodes: &AstNodes<'a>) -> bool {
    if let AstKind::SpreadElement(_) = node.kind() {
      if let Some(parent_node) = nodes.parent_node(node.id()) {
        if let AstKind::ObjectExpression(_) = parent_node.kind() {
          return true;
        }
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::SpreadInObjectLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_detect_spread_in_object_literals:{
      setup: SpreadInObjectLiterals::default(),
      source_code: r#"
        const obj1 = { foo: "bar", x: 42 };
        const obj2 = { bar: "baz", y: 13 };
        const mergedObj = { ...obj1, ...obj2 };
      "#,
      eq: [
        r#"...obj1"#,
        r#"...obj2"#,
      ],
      ne: []
    },

    should_not_detect_spread_in_array_literals:{
      setup: SpreadInObjectLiterals::default(),
      source_code: r#"
        const arr1 = [1, 2, 3];
        const arr2 = [...arr1, 4, 5];
      "#,
      eq: [],
      ne: [
        r#"...arr1"#,
      ]
    }
  }
}
