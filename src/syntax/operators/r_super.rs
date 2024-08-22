use oxc_ast::ast::Expression;

use crate::create_compat;

create_compat! {
  "./r_super.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_call_expression.push(walk_call_expression);
    v.walk_static_member_expression.push(walk_static_member_expression);
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
