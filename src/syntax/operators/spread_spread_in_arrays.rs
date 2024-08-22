use oxc_ast::{ast::SpreadElement, AstKind};

use crate::create_compat;

create_compat! {
  "./spread_spread_in_arrays.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_spread_element.push(walk_spread_element);
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
