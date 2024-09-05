use crate::create_compat_2;

create_compat_2! {
  ObjectInitializer,
  compat {
    name: "operators.object_initializer",
    description: "对象初始化器",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer",
    tags: ["web-features:snapshot:ecmascript-1"],
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
    matches!(node.kind(), AstKind::ObjectExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::ObjectInitializer;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_object_initializer:{
      setup: ObjectInitializer::default(),
      source_code: r#"
        const object1 = { a: 'foo', b: 42, c: {} };
        let object2;
        object2 = { a: 'foo', b: 42, c: {} };
      "#,
      eq: [
        r#"{ a: 'foo', b: 42, c: {} }"#,
        r#"{}"#,
        r#"{ a: 'foo', b: 42, c: {} }"#,
        r#"{}"#,
      ],
      ne: []
    }
  }
}
