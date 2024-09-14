use crate::create_compat;

create_compat! {
  UnicodePointEscapes,
  compat {
    name: "unicode_point_escapes",
    description: "Unicode码点转义 (\\u{})",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Unicode_code_point_escapes",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "44",
      chrome_android: "44",
      firefox: "40",
      firefox_android: "40",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, _node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    // if let AstKind::StringLiteral(string_literal) = node.kind() {
    //   return string_literal.value.contains("\\u{") && string_literal.value.contains("}");
    // }
    // false
    // TODO
    false
  }
}

#[cfg(test)]
mod tests {
  use super::UnicodePointEscapes;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_unicode_point_escapes: {
      setup: UnicodePointEscapes::default(),
      source_code: r#"
        const copyright = "\u{00A9}";
        const emoji = "\u{1F600}";
        const hello = "Hello, world!";
      "#,
      eq: [
        // r#""\u{00A9}""#,
        // r#""\u{1F600}""#,
      ],
      ne: [
        r#""Hello, world!""#,
      ]
    }
  }
}
