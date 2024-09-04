use crate::create_compat_2;

create_compat_2! {
  ForOfAsyncIterators,
  compat {
    name: "for_of_async_iterators",
    description: "for await...of 异步迭代器",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/for-await...of",
    tags: [
      "web-features:snapshot:ecmascript-2018"
    ],
    support: {
      chrome: "63.0.0",
      chrome_android: "63.0.0",
      firefox: "57.0.0",
      firefox_android: "57.0.0",
      safari: "11.1.0",
      safari_ios: "11.3.0",
      edge: "79.0.0",
      node: "10.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::ForOfStatement(for_of) = node.kind() {
      for_of.r#await
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::ForOfAsyncIterators;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_for_await_of:{
      setup: ForOfAsyncIterators::default(),
      source_code: r#"
        async function* asyncGenerator() {
          yield await Promise.resolve(1);
          yield await Promise.resolve(2);
          yield await Promise.resolve(3);
        }
        
        (async () => {
          for await (const num of asyncGenerator()) {
            console.log(num);
          }
        })();
      "#,
      eq: [
        r#"for await (const num of asyncGenerator()) {
            console.log(num);
          }"#,
      ],
      ne: []
    },
    should_ng_when_not_use_for_await_of:{
      setup: ForOfAsyncIterators::default(),
      source_code: r#"
        for (const num of [1, 2, 3]) {
          console.log(num);
        }
      "#,
      eq: [],
      ne: [
        r#"for (const num of [1, 2, 3]) {
          console.log(num);
        }"#,
      ]
    }
  }
}
