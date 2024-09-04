use crate::create_compat_2;

create_compat_2! {
  FunctionsDeclarations,
  compat {
    name: "functions",
    description: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes#defining_classes",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
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
    matches!(node.kind(), AstKind::Function(functions) if functions.is_declaration())
  }
}

#[cfg(test)]
mod tests {

  use super::FunctionsDeclarations;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: FunctionsDeclarations::default(),
      source_code: r#"
        function a() { }
        const b = function() { }
        const c = () => { }
      "#,
      eq: [
        r#"function a() { }"#
      ],
      ne: [
        r#"function() { }"#
      ]
    }
  }
}
