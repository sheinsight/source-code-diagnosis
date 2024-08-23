use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_pattern.push(walk_object_pattern);
  },
  compat {
    name: "rest_in_objects",
    description: "Rest in objects",
    tags: ["web-features:snapshot:ecmascript-2015"],
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
  walk_object_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectPattern| {
    it.rest.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "rest_in_objects",
    setup,

    should_ok_when_use_rest_in_objects,
    r#"
      const {a, ...b} = object;
    "#,
    1,
  }
}
