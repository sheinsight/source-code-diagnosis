use oxc_ast::{ast::SpreadElement, AstKind};

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_spread_element.push(walk_spread_element);
  },
  compat {
    name: "spread_in_arrays",
    description: "Spread in array literals",
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
  |ctx: &mut Context, it: &SpreadElement| {
    if let Some(p) = ctx.stack.last() {
      matches!(p, AstKind::ArrayExpressionElement(_))
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "spread_in_arrays",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
      const parts = ["shoulders", "knees"];
      const lyrics = ["head", ...parts, "and", "toes"];
    "#,
    1
  }
}
