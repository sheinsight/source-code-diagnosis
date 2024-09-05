use crate::create_compat_2;

create_compat_2! {
  BinaryNumericLiterals,
  compat {
    name: "binary_numeric_literals",
    description: "Binary numeric literals",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Binary",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "41",
      chrome_android: "41",
      firefox: "25",
      firefox_android: "25",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::NumericLiteral(numeric_literal) = node.kind() {
      return vec!["0b", "0B"]
        .iter()
        .any(|item| numeric_literal.raw.starts_with(*item))
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::BinaryNumericLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_binary_numeric_literals:{
      setup: BinaryNumericLiterals::default(),
      source_code: r#"
        const binary = 0b1010;
        const anotherBinary = 0B1010;
      "#,
      eq: [
        r#"0b1010"#,
        r#"0B1010"#,
      ],
      ne: [
        r#"1010"#,
      ]
    }
  }
}
