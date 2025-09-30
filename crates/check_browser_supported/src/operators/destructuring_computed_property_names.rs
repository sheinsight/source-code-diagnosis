use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  DestructuringComputedPropertyNames,
  compat {
    name: "operators.destructuring.computed_property_names",
    description: "解构赋值中的计算属性名",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment#Computed_property_names_and_destructuring",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "41",
      firefox_android: "41",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::ObjectPattern(object_pattern) = node.kind() {
      object_pattern.properties.iter().any(|prop| prop.computed)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::DestructuringComputedPropertyNames;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_detect_computed_property_names_in_destructuring:{
      setup: DestructuringComputedPropertyNames::default(),
      source_code: r#"
        const key = "z";
        const { [key]: a } = obj;
      "#,
      eq: [
        r#"{ [key]: a }"#,
      ],
      ne: []
    },

    should_not_detect_regular_destructuring:{
      setup: DestructuringComputedPropertyNames::default(),
      source_code: r#"
        const { a, b } = obj;
      "#,
      eq: [],
      ne: [
        r#"{ a, b }"#,
      ]
    }
  }
}
