use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  HexadecimalNumericLiterals,
  compat {
    name: "hexadecimal_numeric_literals",
    description: "十六进制数字字面量 (0xAF)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Hexadecimal",
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
        return vec!["0x", "0X"].iter().any(|item| raw.starts_with(item));
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::HexadecimalNumericLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_hexadecimal_numeric_literals:{
      setup: HexadecimalNumericLiterals::default(),
      source_code: r#"
        0xFFFFFFFFFFFFFFFFF; // 295147905179352830000
        0x123456789ABCDEF;   // 81985529216486900
        0XA;                 // 10
      "#,
      eq: [
        r#"0xFFFFFFFFFFFFFFFFF"#,
        r#"0x123456789ABCDEF"#,
        r#"0XA"#,
      ],
      ne: []
    }
  }
}
