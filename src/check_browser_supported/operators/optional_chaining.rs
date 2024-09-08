use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  OperatorsOptionalChaining,
  compat {
    name: "operators.optional_chaining",
    description: "可选链操作符 (<code>?.</code>)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Optional_chaining",
    tags: ["web-features:snapshot:ecmascript-2020"],
    support: {
      chrome: "80",
      chrome_android: "80",
      firefox: "74",
      firefox_android: "74",
      safari: "13.1",
      safari_ios: "13.1",
      edge: "80",
      node: "14.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ChainExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsOptionalChaining;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_optional_chaining:{
      setup: OperatorsOptionalChaining::default(),
      source_code: r#"
        const user = {
          name: '1',
          address: {
            city: '2'
          }
        };
        console.log(user?.address?.city);
        console.log(user?.contact?.email);
      "#,
      eq: [
        r#"user?.address?.city"#,
        r#"user?.contact?.email"#,
      ],
      ne: []
    }
  }
}
