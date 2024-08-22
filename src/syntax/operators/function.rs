use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
  "./function.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_function.push(walk_function);
  },

  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &ScopeFlags, is_strict_mode: bool| {
    it.is_expression()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_function",
    setup,

    should_ok_when_function_expression,
    r#"
    let greet = function(name) {
        console.log("Hello, " + name);
    };
    greet("Alice");
    "#,
    1,

    should_ok_when_iife,
    r#"
    (function() {
        console.log("This function is executed immediately.");
    })();
    "#,
    1,
  }
}
