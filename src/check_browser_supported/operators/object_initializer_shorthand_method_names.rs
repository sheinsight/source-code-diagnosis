use oxc_ast::ast::ObjectPropertyKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_expression.push(walk_object_expression);
  },
  compat {
    name: "operators_object_initializer_shorthand_method_names",
    description: "Shorthand method names",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "47",
      chrome_android: "47",
      firefox: "34",
      firefox_android: "34",
      opera: "47",
      opera_android: "47",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      oculus: "47",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_object_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectExpression| {
    it.properties.iter().any(|p| {
      matches!(p, ObjectPropertyKind::ObjectProperty(op) if op.method)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_object_initializer_shorthand_method_names",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    const obj = {
      sayHello() {
        console.log('hello ');
      }
    };
    obj.sayHello();
    "#,
    1
  }
}
