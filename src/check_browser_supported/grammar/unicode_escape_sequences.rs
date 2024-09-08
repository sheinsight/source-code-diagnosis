use crate::create_compat_2;

create_compat_2! {
  UnicodeEscapeSequences,
  compat {
    name: "unicode_escape_sequences",
    description: "Unicode转义序列 ('\\u00A9')",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Unicode_escape_sequences",
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
  fn handle<'a>(&self, _source_code: &str, _node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    // if let AstKind::StringLiteral(string_literal) = node.kind() {
    //   // return string_literal.value.contains("\\u")
    //   // false;
    // }
    // TODO
    false
  }
}

#[cfg(test)]
mod tests {
  use super::UnicodeEscapeSequences;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_unicode_escape_sequences:{
      setup: UnicodeEscapeSequences::default(),
      source_code: r#"
        const copyright = "\u00A9";
        const emoji = "\u{1F600}";
        const hello = "Hello, world!";
      "#,
      eq: [
        // r#""\u00A9""#,
        // r#""\u{1F600}""#,
      ],
      ne: [
        r#""Hello, world!""#,
      ]
    }
  }
}
