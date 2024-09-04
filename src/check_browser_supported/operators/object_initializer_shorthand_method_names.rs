use crate::create_compat_2;

create_compat_2! {
  ObjectInitializerShorthandMethodNames,
  compat {
    name: "operators_object_initializer_shorthand_method_names",
    description: "对象初始化器中的简写方法名",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Functions/Method_definitions",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "47.0.0",
      chrome_android: "47.0.0",
      firefox: "34.0.0",
      firefox_android: "34.0.0",
      safari: "9.0.0",
      safari_ios: "9.0.0",
      edge: "12.0.0",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    matches!(node.kind(), AstKind::ObjectProperty(object_property) if object_property.method)

  }
}

#[cfg(test)]
mod tests {
  use super::ObjectInitializerShorthandMethodNames;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_shorthand_method_names:{
      setup: ObjectInitializerShorthandMethodNames::default(),
      source_code: r#"
        const obj = { sayHello() { } };
        obj.sayHello();
      "#,
      eq: [
        r#"sayHello() { }"#,
      ],
      ne: []
    },

    should_ng_when_not_use_shorthand_method_names:{
      setup: ObjectInitializerShorthandMethodNames::default(),
      source_code: r#"
        const obj = {
          sayHello: function() { }
        };
        obj.sayHello();
      "#,
      eq: [],
      ne: [
        r#"sayHello: function() { }"#,
      ]
    }
  }
}
