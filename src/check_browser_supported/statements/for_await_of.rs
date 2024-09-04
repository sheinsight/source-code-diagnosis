use crate::create_compat_2;

create_compat_2! {
  ForAwaitOf,
  compat {
    name: "for_await_of",
    description: "for await...of 语句",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of",
    tags: ["web-features:snapshot:ecmascript-2018"],
    support: {
      chrome: "63.0.0",
      chrome_android: "63.0.0",
      firefox: "57.0.0",
      firefox_android: "57.0.0",
      safari: "12.0.0",
      safari_ios: "12.0.0",
      edge: "63.0.0",
      node: "10.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ForOfStatement(for_of) if for_of.r#await)
  }
}

#[cfg(test)]
mod tests {
  use super::ForAwaitOf;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_for_await_of_statement:{
      setup: ForAwaitOf::default(),
      source_code: r#"
async function* foo() {
  yield 1;
  yield 2;
}
(async function () {
  for await (const num of foo()) { }
})();
      "#,
      eq: [
        r#"for await (const num of foo()) { }"#,
      ],
      ne: []
    },

    should_not_ok_when_regular_for_of_statement:{
      setup: ForAwaitOf::default(),
      source_code: r#"
function* foo() {
  yield 1;
  yield 2;
}
for (const num of foo()) {
  console.log(num);
}
      "#,
      eq: [],
      ne: [
        r#"for (const num of foo()) {"#,
      ]
    }
  }
}
