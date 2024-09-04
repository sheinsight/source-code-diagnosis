use crate::create_compat_2;

create_compat_2! {
  GeneratorFunctionIteratorResultObject,
  compat {
    name: "generator_function_IteratorResult_object",
    description: "生成器函数返回 IteratorResult 对象而不是抛出异常",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Generator",
    tags: ["web-features:snapshot:ecmascript-2016"],
    support: {
      chrome: "49.0.0",
      chrome_android: "49.0.0",
      firefox: "29.0.0",
      firefox_android: "29.0.0",
      safari: "10.0.0",
      safari_ios: "10.0.0",
      edge: "13.0.0",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      function.generator
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GeneratorFunctionIteratorResultObject;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_generator_function_declaration:{
      setup: GeneratorFunctionIteratorResultObject::default(),
      source_code: r#"
        function* generatorFunction() {
          yield 1;
          yield 2;
          yield 3;
        }
      "#,
      eq: [
        r#"function* generatorFunction() {
          yield 1;
          yield 2;
          yield 3;
        }"#,
      ],
      ne: []
    },

    should_not_ok_when_normal_function_declaration:{
      setup: GeneratorFunctionIteratorResultObject::default(),
      source_code: r#"
        function normalFunction() {
          return 1;
        }
      "#,
      eq: [],
      ne: [
        r#"function normalFunction() {
          return 1;
        }"#,
      ]
    }
  }
}
