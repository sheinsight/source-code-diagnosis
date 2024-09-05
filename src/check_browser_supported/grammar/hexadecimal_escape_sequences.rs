use crate::create_compat_2;

create_compat_2! {
  HexadecimalEscapeSequences,
  compat {
    name: "hexadecimal_escape_sequences",
    description: "Hexadecimal escape sequences ('\\xA9')",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Lexical_grammar#Hexadecimal_escape_sequences",
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
    // TODO
    if let AstKind::StringLiteral(string_literal) = node.kind() {
      false
    } else {
      false
    }
  }
}

fn is_hex_string(s: &str) -> bool {
  s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
  use super::HexadecimalEscapeSequences;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_hexadecimal_escape_sequences:{
      setup: HexadecimalEscapeSequences::default(),
      source_code: r#"
        const copyright = "\xA9";
        const hello = "Hello, world!";
      "#,
      eq: [
        // r#""\xA9""#,
      ],
      ne: [
        r#""Hello, world!""#,
      ]
    }
  }
}
