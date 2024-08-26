use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_property.push(walk_object_property);
  },
  compat {
    name: "operators_object_initializer_shorthand_property_names",
    description: "Shorthand property names",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "47",
      chrome_android: "47",
      firefox: "33",
      firefox_android: "33",
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
  walk_object_property,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectProperty| {
    it.shorthand
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_object_initializer_shorthand_property_names",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    const x = 10;
    const y = 20;

    const point = {
      x,
      y,
      z: 30
    };
    "#,
    2
  }
}
