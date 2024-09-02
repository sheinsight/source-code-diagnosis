use oxc_ast::ast::ObjectPropertyKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_expression.push(walk_object_expression);
  },
  compat {
    name: "operators_object_initializer_spread_properties",
    description: "Spread properties",
    tags: ["web-features:snapshot:ecmascript-2018"],
    support: {
      chrome: "60",
      chrome_android: "60",
      firefox: "55",
      firefox_android: "55",
      safari: "11.1",
      safari_ios: "11.1",
      edge: "60",
      node: "8.3.0",
      deno: "1.0",
    }
  },
  walk_object_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectExpression| {
    it.properties.iter().any(|p| {
      matches!(p, ObjectPropertyKind::SpreadProperty(_))
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_object_initializer_spread_properties",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    const obj1 = { a: 1, b: 2 };
    const obj2 = { c: 3, ...obj1 };
    "#,
    1
  }
}
