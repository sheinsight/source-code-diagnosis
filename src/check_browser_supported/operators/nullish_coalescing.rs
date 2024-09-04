use oxc_syntax::operator::LogicalOperator;

use crate::create_compat_2;

create_compat_2! {
  OperatorsNullishCoalescing,
  compat {
    name: "operators_nullish_coalescing",
    description: "空值合并运算符 (??)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_operator",
    tags: ["web-features:snapshot:ecmascript-2020"],
    support: {
      chrome: "80.0.0",
      chrome_android: "80.0.0",
      firefox: "72.0.0",
      firefox_android: "72.0.0",
      safari: "13.1.0",
      safari_ios: "13.1.0",
      edge: "80.0.0",
      node: "14.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::LogicalExpression(expr) if expr.operator == LogicalOperator::Coalesce)
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsNullishCoalescing;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_nullish_coalescing:{
      setup: OperatorsNullishCoalescing::default(),
      source_code: r#"
        const foo = null ?? 'default string';
        console.log(foo);
        const bar = undefined ?? 42;
      "#,
      eq: [
        r#"null ?? 'default string'"#,
        r#"undefined ?? 42"#,
      ],
      ne: []
    }
  }
}
