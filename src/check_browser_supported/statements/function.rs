use crate::create_compat_2;

create_compat_2! {
  FunctionsDeclarations,
  compat {
    name: "statements.functions",
    description: "function 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/function",
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
    matches!(node.kind(), AstKind::Function(function) if function.is_declaration())
  }
}

#[cfg(test)]
mod tests {
  use super::FunctionsDeclarations;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_function_declaration:{
      setup: FunctionsDeclarations::default(),
      source_code: r#"
        function calcRectArea(width, height) {
          return width * height;
        }
      "#,
      eq: [
        r#"function calcRectArea(width, height) {
          return width * height;
        }"#
      ],
      ne: []
    }
  }
}
