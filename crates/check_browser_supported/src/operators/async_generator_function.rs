use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  AsyncGeneratorFunction,
  compat {
    name: "operators.async_generator_function",
    description: "async function* 表达式",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function*",
    tags: [
      "web-features:snapshot:ecmascript-2018"
    ],
    support: {
      chrome: "63",
      chrome_android: "63",
      firefox: "55",
      firefox_android: "55",
      safari: "12",
      safari_ios: "12",
      edge: "79",
      node: "10.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      return function.is_expression() && function.r#async && function.generator;
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::AsyncGeneratorFunction;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_async_generator_function_expression:{
      setup: AsyncGeneratorFunction::default(),
      source_code: r#"
        const asyncGenerator = async function* () {
          yield await Promise.resolve(1);
          yield await Promise.resolve(2);
          yield await Promise.resolve(3);
        };
      "#,
      eq: [
        r#"async function* () {
          yield await Promise.resolve(1);
          yield await Promise.resolve(2);
          yield await Promise.resolve(3);
        }"#,
      ],
      ne: []
    },

    should_not_ok_when_async_function_expression:{
      setup: AsyncGeneratorFunction::default(),
      source_code: r#"
        const asyncFunc = async function () {
          await Promise.resolve(1);
        };
      "#,
      eq: [],
      ne: [
        r#"async function () {
          await Promise.resolve(1);
        }"#,
      ]
    },

    should_not_ok_when_generator_function_expression:{
      setup: AsyncGeneratorFunction::default(),
      source_code: r#"
        const generator = function* () {
          yield 1;
        };
      "#,
      eq: [],
      ne: [
        r#"function* () {
          yield 1;
        }"#,
      ]
    }
  }
}
