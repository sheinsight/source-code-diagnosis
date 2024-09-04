use oxc_syntax::operator::UnaryOperator;

use crate::create_compat_2;

create_compat_2! {
  Delete,
  compat {
    name: "delete",
    description: "delete 运算符",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/delete",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1.0.0",
      chrome_android: "18.0.0",
      firefox: "1.0.0",
      firefox_android: "4.0.0",
      safari: "1.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::UnaryExpression(expr) if expr.operator == UnaryOperator::Delete)
  }
}

#[cfg(test)]
mod tests {
  use super::Delete;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_delete_object_key:{
      setup: Delete::default(),
      source_code: r#"
        const myObject = {x: 1, y: 2};
        delete myObject.x;
      "#,
      eq: [
        r#"delete myObject.x"#
      ],
      ne: []
    },

    should_not_ok_when_not_delete_operator:{
      setup: Delete::default(),
      source_code: r#"
        const myObject = {x: 1, y: 2};
        myObject.x = undefined;
      "#,
      eq: [],
      ne: [
        r#"myObject.x = undefined"#
      ]
    }
  }
}
