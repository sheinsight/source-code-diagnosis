use crate::create_compat_2;

create_compat_2! {
  ObjectInitializerShorthandPropertyNames,
  compat {
    name: "object_initializer_shorthand_property_names",
    description: "对象字面量的简写属性名",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer#Property_definitions",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "47.0.0",
      chrome_android: "47.0.0",
      firefox: "33.0.0",
      firefox_android: "33.0.0",
      safari: "9.0.0",
      safari_ios: "9.0.0",
      edge: "12.0.0",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(
      node.kind(),
      AstKind::ObjectProperty(object_property) if object_property.shorthand
    )
  }
}

#[cfg(test)]
mod tests {
  use super::ObjectInitializerShorthandPropertyNames;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_detect_shorthand_property_names:{
      setup: ObjectInitializerShorthandPropertyNames::default(),
      source_code: r#"
        const x = 10;
        const y = 20;

        const point = {
          x,
          y,
          z: 30
        };
      "#,
      eq: [
        r#"x"#,
        r#"y"#,
      ],
      ne: [
        r#"z: 30"#,
      ]
    }
  }
}
