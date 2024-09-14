use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  GeneratorFunctionTrailingComma,
  compat {
    name: "operators.generator_function.trailing_comma",
    description: "生成器函数参数中的尾随逗号",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#trailing_commas_in_functions",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "58",
      chrome_android: "58",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "58",
      node: "8.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      if function.generator && !function.r#async {
        let params_source = &source_code[function.params.span.start as usize..function.params.span.end as usize];
        params_source.trim().ends_with(",)")
      } else {
        false
      }
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GeneratorFunctionTrailingComma;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_generator_function_has_trailing_comma: {
      setup: GeneratorFunctionTrailingComma::default(),
      source_code: r#"
        const foo = function* (a, b,) { yield 'a'; };
      "#,
      eq: [
        r#"function* (a, b,) { yield 'a'; }"#,
      ],
      ne: []
    },

    should_not_ok_when_regular_function_has_trailing_comma: {
      setup: GeneratorFunctionTrailingComma::default(),
      source_code: r#"
        const foo = function (a, b,) { return a + b; };
      "#,
      eq: [],
      ne: [
        r#"(a, b,)"#,
      ]
    }
  }
}
