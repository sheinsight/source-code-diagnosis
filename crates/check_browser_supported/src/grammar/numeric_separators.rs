use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  NumericSeparators,
  compat {
    name: "numeric_separators",
    description: "Numeric separators (1_000_000_000_000)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#numeric_separators",
    tags: [
      "web-features:snapshot:ecmascript-2021"
    ],
    support: {
      chrome: "75",
      chrome_android: "75",
      firefox: "70",
      firefox_android: "70",
      safari: "13",
      safari_ios: "13",
      edge: "75",
      node: "12.5.0",
      deno: "1.2.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::NumericLiteral(numeric_literal) = node.kind() {
      if let Some(raw) = &numeric_literal.raw {
        return raw.contains('_');
      }
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
