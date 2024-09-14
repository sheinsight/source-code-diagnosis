use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  AsyncFunction,
  compat {
    name: "operators.async_function",
    description: "async function 表达式",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Statements/async_function",
    tags: ["web-features:async-await"],
    support: {
      chrome: "55",
      chrome_android: "55",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "15",
      node: "7.6.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    match node.kind() {
      AstKind::Function(function) => {
        function.is_expression() && function.r#async && !function.generator
      },
      AstKind::ArrowFunctionExpression(arrow_function) => {
        arrow_function.r#async
      },
      _ => false,
    }
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
        const asyncFunction = async function() { };
      "#,
      eq: [
        r#"async function() { }"#,
      ],
      ne: []
    },

    should_ok_when_async_arrow_function:{
      setup: AsyncFunction::default(),
      source_code: r#"
        const asyncArrowFunction = async () => { };
      "#,
      eq: [
        r#"async () => { }"#,
      ],
      ne: []
    },

    should_not_ok_when_sync_function:{
      setup: AsyncFunction::default(),
      source_code: r#"
        function syncFunction() { }
      "#,
      eq: [],
      ne: [
        r#"function syncFunction() { }"#,
      ]
    },

    should_not_ok_when_async_generator_function:{
      setup: AsyncFunction::default(),
      source_code: r#"
        const asyncGeneratorFunction = async function*() { };
      "#,
      eq: [],
      ne: [
        r#"async function*() { }"#,
      ]
    }
  }
}
