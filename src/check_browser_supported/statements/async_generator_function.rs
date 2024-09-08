use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  AsyncGeneratorFunction,
  compat {
    name: "statements.async_generator_function",
    description: "async function* 语句",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/async_function*",
    tags: ["web-features:snapshot:ecmascript-2018"],
    support: {
      chrome: "63",
      chrome_android: "63",
      firefox: "55",
      firefox_android: "55",
      safari: "12",
      safari_ios: "12",
      edge: "63",
      node: "10.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      function.is_declaration() && function.r#async && function.generator
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::AsyncGeneratorFunction;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_async_generator_function_declaration:{
      setup: AsyncGeneratorFunction::default(),
      source_code: r#"
        async function* foo() {
          yield await Promise.resolve(1);
        }
      "#,
      eq: [
        r#"async function* foo() {
          yield await Promise.resolve(1);
        }"#,
      ],
      ne: []
    },

    should_fail_when_async_function_declaration:{
      setup: AsyncGeneratorFunction::default(),
      source_code: r#"
        async function foo() {
          await Promise.resolve(1);
        }
      "#,
      eq: [],
      ne: [
        r#"async function foo() {
          await Promise.resolve(1);
        }"#,
      ]
    },

    should_fail_when_generator_function_declaration:{
      setup: AsyncGeneratorFunction::default(),
      source_code: r#"
        function* foo() {
          yield 1;
        }
      "#,
      eq: [],
      ne: [
        r#"function* foo() {
          yield 1;
        }"#,
      ]
    },

    should_fail_when_async_generator_function_expression:{
      setup: AsyncGeneratorFunction::default(),
      source_code: r#"
        const foo = async function* () {
          yield await Promise.resolve(1);
        };
      "#,
      eq: [],
      ne: [
        r#"async function* () {
          yield await Promise.resolve(1);
        }"#,
      ]
    }
  }
}
