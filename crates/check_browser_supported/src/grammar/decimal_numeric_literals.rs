use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  DecimalNumericLiterals,
  compat {
    name: "decimal_numeric_literals",
    description: "Decimal numeric literals (1234567890)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Decimal",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::NumericLiteral(numeric_literal) = node.kind() {
      if let Some(raw) = &numeric_literal.raw {
        return !raw.starts_with('0');
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::DecimalNumericLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_decimal_numeric_literals:{
      setup: DecimalNumericLiterals::default(),
      source_code: r#"
        1234567890;
        42;
      "#,
      eq: [
        r#"1234567890"#,
        r#"42"#,
      ],
      ne: []
    },

    should_fail_when_not_decimal_numeric_literals:{
      setup: DecimalNumericLiterals::default(),
      source_code: r#"
        0B42;
      "#,
      eq: [],
      ne: [
        r#"0B42"#,
      ]
    }
  }
}
