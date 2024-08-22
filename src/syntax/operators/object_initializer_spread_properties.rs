use oxc_ast::ast::ObjectPropertyKind;

use crate::create_compat;

create_compat! {
  "./object_initializer_spread_properties.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_expression.push(walk_object_expression);
  },

  walk_object_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectExpression| {
    it.properties.iter().any(|p| {
      matches!(p, ObjectPropertyKind::SpreadProperty(_))
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_object_initializer_spread_properties",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    const obj1 = { a: 1, b: 2 };
    const obj2 = { c: 3, ...obj1 };
    "#,
    1
  }
}
