use oxc_ast::ast::Expression;

use crate::create_compat_2;

create_compat_2! {
  OperatorsSuper,
  compat {
    name: "operators_super",
    description: "super 关键字用于调用超类的对应方法。这是 ES6 类语法的关键特性之一。",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/super",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "42.0.0",
      chrome_android: "42.0.0",
      firefox: "45.0.0",
      firefox_android: "45.0.0",
      safari: "7.0.0",
      safari_ios: "7.0.0",
      edge: "13.0.0",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    match node.kind() {
      AstKind::CallExpression(call_expr) => {
        matches!(call_expr.callee, Expression::Super(_))
      },
      AstKind::MemberExpression(static_member_expr) => {
        matches!(static_member_expr.object(), Expression::Super(_))
      },
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::OperatorsSuper;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_super_in_call_expression:{
      setup: OperatorsSuper::default(),
      source_code: r#"
        class Animal {
          constructor(name) {
            this.name = name;
          }
        }
        class Dog extends Animal {
          constructor(name, breed) {
            super(name);
            this.breed = breed;
          }
        }
      "#,
      eq: [
        r#"super(name)"#,
      ],
      ne: []
    },

    should_ok_when_use_super_in_static_member_expression:{
      setup: OperatorsSuper::default(),
      source_code: r#"
        class FooBar extends Foo {
          getFullName() {
            return this.name + super.getNameSeparator() + this.index;
          }
        }
      "#,
      eq: [
        r#"super.getNameSeparator"#,
      ],
      ne: []
    }
  }
}
