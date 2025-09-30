use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
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
