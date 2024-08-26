use oxc_ast::ast::FunctionType;
use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "function",
    description: "function 语句",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags| {
    matches!(it.r#type, FunctionType::FunctionDeclaration)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "function",
    setup,

    should_ok_when_function_declaration,
    r#"
    function calcRectArea(width, height) {
      return width * height;
    }
    "#,
    1
  }
}
