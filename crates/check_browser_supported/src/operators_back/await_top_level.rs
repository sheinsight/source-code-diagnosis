use oxc::ast::AstKind;

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
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_await_expression.push(walk_await_expression);
  },
  compat {
    name: "await_top_level",
    description: "Use at module top level",
    tags: ["web-features:async-await"],
    support: {
      chrome: "89",
      chrome_android: "89",
      firefox: "89",
      firefox_android: "89",
      safari: "15",
      safari_ios: "15",
      edge: "89",
      node: "14.8.0",
      deno: "1.0",
    }
  },
  walk_await_expression,
  |ctx: &mut Context, it: &oxc::ast::ast::AwaitExpression| {
    is_top_level_await(&ctx.stack)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

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
