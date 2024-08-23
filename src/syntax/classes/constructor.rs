use oxc_ast::ast::ClassElement;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_class_body.push(walk_class_body);
  },
  compat {
    name: "classes_constructor",
    description: "constructor function",
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
  walk_class_body,
  |ctx: &mut Context, it: &oxc_ast::ast::ClassBody| {
    it.body.iter().any(|item| {
      if let ClassElement::MethodDefinition(method_definition) = item {
        if let Some(name) = method_definition.key.name() {
          return name == "constructor";
        }
      }
      false
    })
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_constructor",
    setup,

    should_ok_when_use_class_constructor,
    r#"
      class A { constructor() { } }
    "#,
    1,
  }
}
