use oxc_ast::ast::Expression;

use crate::create_compat;

create_compat! {
  "./object_initializer.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_assignment_expression.push(walk_assignment_expression);
    v.walk_variable_declaration.push(walk_variable_declaration);
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.right, Expression::ObjectExpression(_))
  },
  walk_variable_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::VariableDeclaration| {
    it.declarations.iter().any(|decl| {
      decl.init.as_ref().map_or(false, |init| matches!(init, Expression::ObjectExpression(_)))
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_object_initializer",
    setup,
    should_ok_when_variable_declaration_object_initializer,
    r#"
    const object1 = { a: 'foo', b: 42, c: {} };
    "#,
    1,

    should_ok_when_assignment_expression_object_initializer,
    r#"
    const object2 = { a: 'foo', b: 42, c: {} };
    object2 = { a: 'foo', b: 42, c: {} };
    "#,
    2,
  }
}
