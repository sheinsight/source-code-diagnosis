use oxc_ast::{ast::SpreadElement, AstKind};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_spread_element.push(walk_spread_element);
  },
  compat {
    name: "spread_in_object_literals",
    description: "Spread in object literals",
    tags: [],
    support: {
      chrome: "60",
      chrome_android: "60",
      firefox: "55",
      firefox_android: "55",
      opera: "60",
      opera_android: "60",
      safari: "11.1",
      safari_ios: "11.1",
      edge: "60",
      oculus: "60",
      node: "8.3.0",
      deno: "1.0",
    }
  },
  walk_spread_element,
  |ctx: &mut Context, _it: &SpreadElement| {
    ctx.stack
      .last()
      .map_or(false, |p| matches!(p, AstKind::ObjectExpression(_)))
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "spread_in_object_literals",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      const obj1 = { foo: "bar", x: 42 };
      const obj2 = { bar: "baz", y: 13 };
      const mergedObj = { ...obj1, ...obj2 };
    "#,
    2
  }
}
