use crate::create_compat_2;

create_compat_2! {
  AsyncFunction,
  compat {
    name: "async_function",
    description: "async function 语句",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/async_function",
    tags: [
      "web-features:async-await"
    ],
    support: {
      chrome: "55.0.0",
      chrome_android: "55.0.0",
      firefox: "52.0.0",
      firefox_android: "52.0.0",
      safari: "10.1.0",
      safari_ios: "10.1.0",
      edge: "15.0.0",
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
