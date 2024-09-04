use crate::create_compat_2;

create_compat_2! {
  FunctionTrailingComma,
  compat {
    name: "functions_trailing_comma",
    description: "函数参数中的尾随逗号",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#trailing_commas_in_functions",
    tags: [
      "web-features:snapshot:ecmascript-2017"
    ],
    support: {
      chrome: "58.0.0",
      chrome_android: "58.0.0",
      firefox: "52.0.0",
      firefox_android: "52.0.0",
      safari: "10.0.0",
      safari_ios: "10.0.0",
      edge: "14.0.0",
      node: "8.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      let params_source = &source_code[function.params.span.start as usize..function.params.span.end as usize];
      params_source.trim().ends_with(",)")
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::FunctionTrailingComma;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_function_has_trailing_comma: {
      setup: FunctionTrailingComma::default(),
      source_code: r#"
        let greet = function(name, age,) { };
        greet("Alice", 30);
      "#,
      eq: [
        r#"function(name, age,) { }"#,
      ],
      ne: []
    },

    should_not_ok_when_function_has_no_trailing_comma: {
      setup: FunctionTrailingComma::default(),
      source_code: r#"
        let greet = function(name, age) { };
        greet("Alice", 30);
      "#,
      eq: [],
      ne: [
        r#"(name, age)"#,
      ]
    }
  }
}
