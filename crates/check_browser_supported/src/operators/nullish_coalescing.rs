use oxc_ast::AstKind;
use oxc_syntax::operator::LogicalOperator;

use crate::create_compat;

create_compat! {
  OperatorsNullishCoalescing,
  compat {
    name: "operators.nullish_coalescing",
    description: "空值合并运算符 (??)",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_operator",
    tags: ["web-features:snapshot:ecmascript-2020"],
    support: {
      chrome: "80",
      chrome_android: "80",
      firefox: "72",
      firefox_android: "72",
      safari: "13.1",
      safari_ios: "13.1",
      edge: "80",
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
