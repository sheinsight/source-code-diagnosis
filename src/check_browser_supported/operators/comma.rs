use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  OperatorsComma,
  compat {
    name: "operators.comma",
    description: "逗号运算符",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Comma_Operator",
    tags: ["web-features:snapshot:ecmascript-1"],
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
    matches!(node.kind(), AstKind::SequenceExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsComma;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_comma:{
      setup: OperatorsComma::default(),
      source_code: r#"
        let x = 1;
        x = (x++, x);
      "#,
      eq: [
        r#"x++, x"#,
      ],
      ne: []
    }
  }
}
