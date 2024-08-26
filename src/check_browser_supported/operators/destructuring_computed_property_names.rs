use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_pattern.push(walk_object_pattern);
  },
  compat {
    name: "computed_property_names",
    description: "Computed property names",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "41",
      firefox_android: "41",
      opera: "49",
      opera_android: "49",
      safari: "10",
      safari_ios: "10",
      edge: "14",
      oculus: "49",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_object_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectPattern| {
    it.properties.iter().any(|prop| prop.computed)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "computed_property_names",
    setup,

    should_ok_when_use_computed_property_names,
    r#"
      const key = "z";
      const { [key]: a } = obj;
    "#,
    1,
  }
}
