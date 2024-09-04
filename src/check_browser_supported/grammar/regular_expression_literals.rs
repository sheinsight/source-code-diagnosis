use crate::create_compat_2;

create_compat_2! {
  RegularExpressionLiterals,
  compat {
    name: "regular_expression_literals",
    description: "Regular expression literals (/ab+c/g)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_Expressions",
    tags: [
      "web-features:snapshot:ecmascript-3"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "1.0.0",
      firefox: "1.0.0",
      firefox_android: "1.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::RegExpLiteral(_))
  }
}

#[cfg(test)]
mod tests {
  use super::RegularExpressionLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_regular_expression_literals: {
      setup: RegularExpressionLiterals::default(),
      source_code: r#"
        /ab+c/g;
        /[/]/;
      "#,
      eq: [
        r#"/ab+c/g"#,
        r#"/[/]/"#,
      ],
      ne: []
    }
  }
}
