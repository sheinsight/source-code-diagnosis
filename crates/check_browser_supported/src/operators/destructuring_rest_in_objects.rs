use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  DestructuringRestInObjects,
  compat {
    name: "operators.destructuring.rest_in_objects",
    description: "对象中的剩余元素",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment#Rest_in_Object_Destructuring",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "60",
      chrome_android: "60",
      firefox: "55",
      firefox_android: "55",
      safari: "11",
      safari_ios: "11",
      edge: "60",
      node: "8.3.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::ObjectPattern(object_pattern) = node.kind() {
      object_pattern.rest.is_some()
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::DestructuringRestInObjects;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_rest_in_objects:{
      setup: DestructuringRestInObjects::default(),
      source_code: r#"
        const {a, ...b} = object;
      "#,
      eq: [
        r#"{a, ...b}"#,
      ],
      ne: []
    },

    should_not_ok_when_not_use_rest_in_objects:{
      setup: DestructuringRestInObjects::default(),
      source_code: r#"
        const {a, b} = object;
      "#,
      eq: [],
      ne: [
        r#"{a, b}"#,
      ]
    }
  }
}
