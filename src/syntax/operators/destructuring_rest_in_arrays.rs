use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_array_pattern.push(walk_array_pattern);
  },
  compat {
    name: "rest_in_arrays",
    description: "Rest in arrays",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "41",
      firefox_android: "41",
      opera: "49",
      opera_android: "49",
      safari: "9.1",
      safari_ios: "9.1",
      edge: "16",
      oculus: "49",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_array_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ArrayPattern| {
    it.rest.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "rest_in_arrays",
    setup,

    should_ok_when_use_rest_in_arrays,
    r#"
      const [a, ...b] = array;
    "#,
    1,
  }
}
