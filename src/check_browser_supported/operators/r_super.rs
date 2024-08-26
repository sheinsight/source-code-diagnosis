use oxc_ast::ast::Expression;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_call_expression.push(walk_call_expression);
    v.walk_static_member_expression.push(walk_static_member_expression);
  },
  compat {
    name: "operators_super",
    description: "The `super` keyword is used to call corresponding methods of super class. This is one of the key features of ES6 class syntax.",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "42",
      chrome_android: "42",
      firefox: "45",
      firefox_android: "45",
      opera: "42",
      opera_android: "42",
      safari: "7",
      safari_ios: "7",
      edge: "13",
      oculus: "42",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_call_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::CallExpression| {
    matches!(it.callee, Expression::Super(_))
  },
  walk_static_member_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::StaticMemberExpression| {
    matches!(it.object, Expression::Super(_))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_super",
    setup,

    should_ok_when_call_expression,
    r#"
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
    1,

    should_ok_when_static_member_expression,
    r#"
      class FooBar extends Foo {
        getFullName() {
          return this.name + super.getNameSeparator() + this.index;
        }
      }
    "#,
    1,
  }
}
