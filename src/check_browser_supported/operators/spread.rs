use oxc_ast::ast::SpreadElement;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_spread_element.push(walk_spread_element);
  },
  compat {
    name: "spread",
    description: "Spread syntax (...)",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "16",
      firefox_android: "16",
      safari: "8",
      safari_ios: "8",
      edge: "12",
      node: "5.0.0",
      deno: "1.0",
    }
  },
  walk_spread_element,
  |ctx: &mut Context, _it: &SpreadElement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "spread",
    setup,
    should_ok_when_spread_in_callee_function,
    r#"
      console.log(sum(...numbers));
    "#,
    1,

    should_ok_when_spread_in_object_pattern,
    r#"
      const obj = { ...true,  ...10 };
    "#,
    2,
  }
}
