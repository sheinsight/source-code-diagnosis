use crate::create_compat_2;

create_compat_2! {
  GeneratorFunctionTrailingCommaInParameters,
  compat {
    name: "generator_function_trailing_comma_in_parameters",
    description: "生成器函数参数中的尾随逗号",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#trailing_commas_in_functions",
    tags: [
      "web-features:snapshot:ecmascript-2015"
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
      function.generator &&
      !function.r#async &&
      function.is_declaration() &&
      source_code[function.params.span.start as usize..function.params.span.end as usize].trim().ends_with(",)")
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GeneratorFunctionTrailingCommaInParameters;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_generator_function_declaration: {
      setup: GeneratorFunctionTrailingCommaInParameters::default(),
      source_code: r#"
        function* calcRectArea(width, height,) {
          return width * height;
        }
      "#,
      eq: [
        r#"function* calcRectArea(width, height,) {
          return width * height;
        }"#,
      ],
      ne: []
    },

    should_not_ok_when_generator_function_expression: {
      setup: GeneratorFunctionTrailingCommaInParameters::default(),
      source_code: r#"
        const calcRectArea = function* (width, height,) {
          return width * height;
        }
      "#,
      eq: [],
      ne: [
        r#"function* (width, height,) {
          return width * height;
        }"#,
      ]
    },

    should_not_ok_when_no_trailing_comma: {
      setup: GeneratorFunctionTrailingCommaInParameters::default(),
      source_code: r#"
        function* calcRectArea(width, height) {
          return width * height;
        }
      "#,
      eq: [],
      ne: [
        r#"function* calcRectArea(width, height) {
          return width * height;
        }"#,
      ]
    },

    should_not_ok_when_async_generator_function: {
      setup: GeneratorFunctionTrailingCommaInParameters::default(),
      source_code: r#"
        async function* calcRectArea(width, height,) {
          return width * height;
        }
      "#,
      eq: [],
      ne: [
        r#"async function* calcRectArea(width, height,) {
          return width * height;
        }"#,
      ]
    }
  }
}
