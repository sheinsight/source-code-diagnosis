use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  OctalNumericLiterals,
  compat {
    name: "octal_numeric_literals",
    description: "Octal numeric literals (0o)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Lexical_grammar#octal_literals",
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
      if let Some(raw) = &numeric_literal.raw {
        return raw.starts_with("0O") || raw.starts_with("0o");
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::OctalNumericLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_octal_numeric_literals:{
      setup: OctalNumericLiterals::default(),
      source_code: r#"
        0O755;
        0o644;
      "#,
      eq: [
        r#"0O755"#,
        r#"0o644"#,
      ],
      ne: []
    }
  }
}
