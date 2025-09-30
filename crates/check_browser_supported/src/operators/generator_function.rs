use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  OperatorsGeneratorFunction,
  compat {
    name: "operators.generator_function",
    description: "<code>function*</code> 表达式",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/function*",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "26",
      firefox_android: "26",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::Function(function) = node.kind() {
      return function.generator && !function.r#async;
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsGeneratorFunction;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_generator_function_expression:{
      setup: OperatorsGeneratorFunction::default(),
      source_code: r#"
        const foo = function* () {
          yield 'a';
          yield 'b';
          yield 'c';
        };
      "#,
      eq: [
        r#"function* () {
          yield 'a';
          yield 'b';
          yield 'c';
        }"#,
      ],
      ne: []
    },
    should_not_ok_when_async_generator_function:{
      setup: OperatorsGeneratorFunction::default(),
      source_code: r#"
        const foo = async function* () {
          yield 'a';
        };
      "#,
      eq: [],
      ne: [
        r#"async function* () {
          yield 'a';
        }"#,
      ]
    }
  }
}
