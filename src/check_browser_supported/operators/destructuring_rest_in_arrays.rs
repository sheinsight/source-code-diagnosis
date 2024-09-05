use crate::create_compat_2;

create_compat_2! {
  DestructuringRestInArrays,
  compat {
    name: "operators.destructuring.rest_in_arrays",
    description: "数组中的剩余元素解构",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Destructuring_assignment#Rest_in_Arrays",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "41",
      firefox_android: "41",
      safari: "9",
      safari_ios: "9",
      edge: "16",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::ArrayPattern(array_pattern) = node.kind() {
      array_pattern.rest.is_some()
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::DestructuringRestInArrays;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_rest_in_arrays:{
      setup: DestructuringRestInArrays::default(),
      source_code: r#"
        const [a, ...b] = array;
      "#,
      eq: [
        r#"[a, ...b]"#,
      ],
      ne: []
    },

    should_not_ok_when_not_use_rest_in_arrays:{
      setup: DestructuringRestInArrays::default(),
      source_code: r#"
        const [a, b] = array;
      "#,
      eq: [],
      ne: [
        r#"[a, b]"#,
      ]
    }
  }
}
