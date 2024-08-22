use oxc_ast::ast::ObjectPropertyKind;

use crate::create_compat;

create_compat! {
  "./object_initializer_shorthand_method_names.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_expression.push(walk_object_expression);
  },

  walk_object_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectExpression| {
    it.properties.iter().any(|p| {
      matches!(p, ObjectPropertyKind::ObjectProperty(op) if op.method)
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_object_initializer_shorthand_method_names",
    setup,

    should_ok_when_async_generator_function_declaration,
    r#"
    const obj = {
      sayHello() {
        console.log('hello ');
      }
    };
    obj.sayHello();
    "#,
    1
  }
}
