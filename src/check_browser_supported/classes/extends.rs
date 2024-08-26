use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_class.push(walk_class);
  },
  compat {
    name: "classes_extends",
    description: "extends",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "45",
      firefox_android: "45",
      opera: "49",
      opera_android: "49",
      safari: "9",
      safari_ios: "9",
      edge: "13",
      oculus: "49",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_class,
  |ctx: &mut Context, it: &oxc_ast::ast::Class| {
    it.super_class.is_some()
  }
}

#[cfg(test)]
mod tests {

  use crate::assert_ok_count;

  use super::setup;

  assert_ok_count! {
    "classes_extends",
    setup,

    should_ok_when_use_class_extends,
    r#"
      class A extends B { }
    "#,
    1,

    should_ok_when_use_two_class_extends,
    r#"
      class A extends B { }
      class C extends D { }
    "#,
    2,

    should_ok_when_not_use_extends,
    r#"
      class H{ }
    "#,
    0,

    should_ok_when_use_class_expression_extends,
    r#"
      const MyClass = class extends BaseClass { };
    "#,
    1,

    should_ok_when_use_extends_with_complex_expression,
    r#"
      class ComplexExtends extends (foo ? Bar : Baz) { }
    "#,
    1,

    should_ok_when_use_extends_with_nested_classes,
    r#"
      class Outer extends OuterBase {
        method() {
          return class Inner extends InnerBase { };
        }
      }
    "#,
    2,

    should_not_ok_when_use_object_literal,
    r#"
      const obj = {
        __proto__: BaseObj,
        method() { }
      };
    "#,
    0,

    should_ok_when_use_multiple_inheritance_simulation,
    r#"
      class Base1 { }
      class Base2 { }
      class Derived extends Base1 {
        constructor() {
          super();
          Object.assign(this, new Base2());
        }
      }
    "#,
    1
  }
}
