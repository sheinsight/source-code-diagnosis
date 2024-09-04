use crate::create_compat_2;

create_compat_2! {
  NumericSeparators,
  compat {
    name: "numeric_separators",
    description: "Numeric separators (1_000_000_000_000)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#numeric_separators",
    tags: [
      "web-features:snapshot:ecmascript-2021"
    ],
    support: {
      chrome: "75.0.0",
      chrome_android: "75.0.0",
      firefox: "70.0.0",
      firefox_android: "70.0.0",
      safari: "13.0.0",
      safari_ios: "13.0.0",
      edge: "75.0.0",
      node: "12.5.0",
      deno: "1.2.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::NumericLiteral(numeric_literal) = node.kind() {
      return numeric_literal.raw.contains('_');
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::NumericSeparators;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_numeric_separators:{
      setup: NumericSeparators::default(),
      source_code: r#"
        const billion = 1_000_000_000;
        const binary = 0b1010_0001_1000_0101;
        const hex = 0xA0_B0_C0;
      "#,
      eq: [
        r#"1_000_000_000"#,
        r#"0b1010_0001_1000_0101"#,
        r#"0xA0_B0_C0"#,
      ],
      ne: [
        r#"1000000000"#,
      ]
    }
  }
}
