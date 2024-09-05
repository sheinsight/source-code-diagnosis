use crate::create_compat_2;

create_compat_2! {
  ShorthandObjectLiterals,
  compat {
    name: "shorthand_object_literals",
    description: "Shorthand syntax for object literals",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer#Property_definitions",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "43",
      chrome_android: "43",
      firefox: "33",
      firefox_android: "33",
      safari: "9",
      safari_ios: "9",
      edge: "12",
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
  use super::ShorthandObjectLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_shorthand_object_literals:{
      setup: ShorthandObjectLiterals::default(),
      source_code: r#"
        const name = "Alice";
        const age = 30;

        const person = {
          name,
          age,
          sayHello() {
            console.log(`Hello, I'm ${this.name}`);
          },
          [`status_${Date.now()}`]: "active"
        };
      "#,
      eq: [
        r#"name"#,
        r#"age"#,
      ],
      ne: [
        r#"sayHello() {"#,
        r#"[`status_${Date.now()}`]: "active""#,
      ]
    }
  }
}
