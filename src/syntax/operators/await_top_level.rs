use oxc_ast::AstKind;

use crate::create_compat;

fn is_top_level_await(stack: &Vec<AstKind>) -> bool {
  match stack.last() {
    Some(AstKind::Program(_))
    | Some(AstKind::ExportDefaultDeclaration(_))
    | Some(AstKind::ImportDeclaration(_))
    | Some(AstKind::ExpressionStatement(_))
    | Some(AstKind::VariableDeclarator(_))
    | Some(AstKind::ReturnStatement(_))
    | Some(AstKind::IfStatement(_)) => true,
    _ => false,
  }
}

create_compat! {
  "./await_top_level.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_await_expression.push(walk_await_expression);
  },

  walk_await_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AwaitExpression| {
    is_top_level_await(&ctx.stack)
  }
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::operators::await_top_level::setup};

  assert_ok_count! {
    "await_top_level",
    setup,

    should_ok_when_await_top_level,
    r#"
      const response = await fetch('https://api.example.com/data');
      const data = await response.json();
    "#,
    2,

    should_ok_when_await_top_level_in_if_statement,
    r#"
      if (true) {
        const response = await fetch('https://api.example.com/data');
        const data = await response.json();
      }
    "#,
    2,
  }
}
