use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  ObjectInitializerSpreadProperties,
  compat {
    name: "operators.object_initializer.spread_properties",
    description: "对象初始化器中的展开属性",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Spread_syntax#spread_in_object_literals",
    tags: ["web-features:snapshot:ecmascript-2018"],
    support: {
      chrome: "60",
      chrome_android: "60",
      firefox: "55",
      firefox_android: "55",
      safari: "11.1",
      safari_ios: "11.1",
      edge: "60",
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
  use super::ObjectInitializerSpreadProperties;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_detect_spread_properties_in_object_literal:{
      setup: ObjectInitializerSpreadProperties::default(),
      source_code: r#"
        const obj1 = { a: 1, b: 2 };
        const obj2 = { c: 3, ...obj1 };
      "#,
      eq: [
        r#"...obj1"#,
      ],
      ne: []
    },

    should_detect_multiple_spread_properties:{
      setup: ObjectInitializerSpreadProperties::default(),
      source_code: r#"
        const obj1 = { a: 1 };
        const obj2 = { b: 2 };
        const obj3 = { ...obj1, c: 3, ...obj2 };
      "#,
      eq: [
        r#"...obj1"#,
        r#"...obj2"#,
      ],
      ne: []
    },

    should_not_detect_spread_in_array_literals:{
      setup: ObjectInitializerSpreadProperties::default(),
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
