use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  GeneratorFunction,
  compat {
    name: "statements.generator_function",
    description: "function* 语句",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/function*",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "39",
      chrome_android: "39",
      firefox: "26",
      firefox_android: "26",
      safari: "10",
      safari_ios: "10",
      edge: "13",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      function.is_declaration() && !function.r#async && function.generator
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::GeneratorFunction;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_generator_function_declaration:{
      setup: GeneratorFunction::default(),
      source_code: r#"
        function* generator(i) {
          yield i;
          yield i + 10;
        }
      "#,
      eq: [
        r#"function* generator(i) {
          yield i;
          yield i + 10;
        }"#,
      ],
      ne: []
    },

    should_fail_when_generator_function_expression:{
      setup: GeneratorFunction::default(),
      source_code: r#"
        const generator = function* (i) {
          yield i;
          yield i + 10;
        }
      "#,
      eq: [],
      ne: [
        r#"function* (i) {
          yield i;
          yield i + 10;
        }"#,
      ]
    },

    should_fail_when_async_generator_function:{
      setup: GeneratorFunction::default(),
      source_code: r#"
        async function* generator(i) {
          yield await i;
          yield await (i + 10);
        }
      "#,
      eq: [],
      ne: [
        r#"async function* generator(i) {
          yield await i;
          yield await (i + 10);
        }"#,
      ]
    }
  }
}
