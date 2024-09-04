use crate::create_compat_2;

create_compat_2! {
  Arguments,
  compat {
    name: "functions.arguments",
    description: "function arguments",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/arguments",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "1.0.0",
      firefox: "1.0.0",
      firefox_android: "1.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::IdentifierReference(identifier_reference) if identifier_reference.name == "arguments")
  }
}

#[cfg(test)]
mod tests {

  use super::Arguments;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: Arguments::default(),
      source_code: r#"
        function func1(a, b, c) {
          console.log(arguments[0]);
          console.log(arguments.length);
          console.log(arguments[1]);
          console.log(arguments[2]);
        }
      "#,
      eq: [
        r#"arguments"#,
        r#"arguments"#,
        r#"arguments"#,
        r#"arguments"#
      ],
      ne: [
        r#"function() { }"#
      ]
    }
  }
}
