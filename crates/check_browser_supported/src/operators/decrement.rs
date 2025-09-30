use oxc::ast::AstKind;
use oxc::syntax::operator::UpdateOperator;

use crate::create_compat;

create_compat! {
  Decrement,
  compat {
    name: "operators.decrement",
    description: "递减运算符 (<code>--</code>)",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Decrement",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "2",
      chrome_android: "2",
      firefox: "1",
      firefox_android: "1",
      safari: "4",
      safari_ios: "4",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::UpdateExpression(update_expr) if matches!(update_expr.operator, UpdateOperator::Decrement))
  }
}

#[cfg(test)]
mod tests {
  use super::Decrement;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_decrement:{
      setup: Decrement::default(),
      source_code: r#"
        let x = 3;
        let y = x--;
        console.log(x);
        console.log(y);
      "#,
      eq: [
        r#"x--"#,
      ],
      ne: []
    }
  }
}
