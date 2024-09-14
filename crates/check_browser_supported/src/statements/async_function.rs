use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  AsyncFunction,
  compat {
    name: "statements.async_function",
    description: "async function 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/async_function",
    tags: [
      "web-features:async-await"
    ],
    support: {
      chrome: "55",
      chrome_android: "55",
      firefox: "52",
      firefox_android: "52",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "15",
      node: "7.6.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      return function.is_declaration() && function.r#async && !function.generator;
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::AsyncFunction;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_async_function_declaration:{
      setup: AsyncFunction::default(),
      source_code: r#"
        async function hello() {}
      "#,
      eq: [
        r#"async function hello() {}"#,
      ],
      ne: []
    },

    should_fail_when_sync_function_declaration:{
      setup: AsyncFunction::default(),
      source_code: r#"
        function hello() {}
      "#,
      eq: [],
      ne: [
        r#"function hello() {}"#,
      ]
    },

    should_fail_when_async_generator_function_declaration:{
      setup: AsyncFunction::default(),
      source_code: r#"
        async function* hello() {}
      "#,
      eq: [],
      ne: [
        r#"async function* hello() {}"#,
      ]
    },

    should_fail_when_async_function_expression:{
      setup: AsyncFunction::default(),
      source_code: r#"
        const hello = async function() {}
      "#,
      eq: [],
      ne: [
        r#"async function() {}"#,
      ]
    }
  }
}
