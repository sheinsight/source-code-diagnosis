use oxc_syntax::operator::AssignmentOperator;

use crate::create_compat_2;

create_compat_2! {
  NullishCoalescingAssignment,
  compat {
    name: "operators_nullish_coalescing_assignment",
    description: "空值合并赋值运算符 (x ??= y)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Nullish_coalescing_assignment",
    tags: ["web-features:snapshot:ecmascript-2021"],
    support: {
      chrome: "85.0.0",
      chrome_android: "85.0.0",
      firefox: "79.0.0",
      firefox_android: "79.0.0",
      safari: "14.0.0",
      safari_ios: "14.0.0",
      edge: "85.0.0",
      node: "15.0.0",
      deno: "1.2.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::AssignmentExpression(expr) if matches!(expr.operator, AssignmentOperator::LogicalNullish))
  }
}

#[cfg(test)]
mod tests {
  use super::NullishCoalescingAssignment;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_nullish_coalescing_assignment:{
      setup: NullishCoalescingAssignment::default(),
      source_code: r#"
        let x = null;
        let y = 5;
        x ??= y;
        console.log(x);
      "#,
      eq: [
        r#"x ??= y"#,
      ],
      ne: []
    }
  }
}
