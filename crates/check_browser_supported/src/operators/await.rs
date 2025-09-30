use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  OperatorsAwait,
  compat {
    name: "operators.await",
    description: "await 运算符",
    mdn_url: "https://developer.mozilla.org/zh-CN/docs/Web/JavaScript/Reference/Operators/await",
    tags: ["web-features:async-await"],
    support: {
      chrome: "55",
      chrome_android: "55",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      node: "7.6.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::AwaitExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsAwait;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_await:{
      setup: OperatorsAwait::default(),
      source_code: r#"
        async function fetchData() {
          const response = await fetch('https://api.example.com/data');
          const data = await response.json();
          return data;
        }
      "#,
      eq: [
        r#"await fetch('https://api.example.com/data')"#,
        r#"await response.json()"#,
      ],
      ne: []
    },

    should_ok_when_use_await_in_arrow_function:{
      setup: OperatorsAwait::default(),
      source_code: r#"
        const fetchData = async () => {
          const response = await fetch('https://api.example.com/data');
          const data = await response.json();
          return data;
        };
      "#,
      eq: [
        r#"await fetch('https://api.example.com/data')"#,
        r#"await response.json()"#,
      ],
      ne: []
    },

    should_ok_when_use_await_in_try_catch_statement:{
      setup: OperatorsAwait::default(),
      source_code: r#"
        async function fetchData() {
          try {
            const response = await fetch('https://api.example.com/data');
            const data = await response.json();
            return data;
          } catch (error) {
            console.error('Error:', error);
          }
        }
      "#,
      eq: [
        r#"await fetch('https://api.example.com/data')"#,
        r#"await response.json()"#,
      ],
      ne: []
    }
  }
}
