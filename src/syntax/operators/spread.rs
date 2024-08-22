use oxc_ast::ast::SpreadElement;

use crate::create_compat;

create_compat! {
  "./spread.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_spread_element.push(walk_spread_element);
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
