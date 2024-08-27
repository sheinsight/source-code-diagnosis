use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_static_member_expression.push(walk_static_member_expression);
    v.walk_computed_member_expression.push(walk_computed_member_expression);
  },
  compat {
    name: "operators_property_accessors",
    description: "Property accessors",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_static_member_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::StaticMemberExpression| {
    true
  },
  walk_computed_member_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ComputedMemberExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_property_accessors",
    setup,

    should_ok_when_computed_member_expression,
    r#"car['brand'];"#,
    1,

    should_ok_when_static_member_expression,
    r#"car.brand;"#,
    1,
  }
}
