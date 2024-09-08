use oxc_ast::AstKind;

use crate::create_compat_2;

create_compat_2! {
  OperatorsNew,
  compat {
    name: "operators.new",
    description: "new 运算符创建一个用户定义的对象类型的实例或具有构造函数的内置对象的实例。",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Operators/new",
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
    matches!(node.kind(), AstKind::NewExpression(_))
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsNew;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_new_operator:{
      setup: OperatorsNew::default(),
      source_code: r#"
        function Car(make, model, year) {
          this.make = make;
          this.model = model;
          this.year = year;
        }

        const car1 = new Car('Eagle', 'Talon TSi', 1993);
      "#,
      eq: [
        r#"new Car('Eagle', 'Talon TSi', 1993)"#,
      ],
      ne: []
    }
  }
}
