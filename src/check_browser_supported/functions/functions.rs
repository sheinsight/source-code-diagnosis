use oxc_ast::ast::Function;
use oxc_semantic::ScopeFlags;

use crate::create_compat;

create_compat! {
    setup,
    |v: &mut SyntaxVisitor| {
        v.walk_function.push(walk_function);
    },
    compat {
        name: "function_name",
        description: "function.name property",
        tags: [
            "web-features:function-name",
            "web-features:snapshot:ecmascript-2015"
        ],
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
    |ctx: &mut Context, it: &Function, flags: &ScopeFlags| {
      it.is_declaration()
    }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
      "function_name",
      setup,

      should_ok_when_use_function_name,
      r#"
          function foo() {}
          console.log(foo.name);
        "#,
      1,

      should_ok_when_not_use_function_name,
      r#"
          const foo = function() {};
          console.log(foo.name);
        "#,
      0,

      should_ok_when_use_function_name_in_assignment,
      r#"
          const bar = function foo() {};
          console.log(bar.name);
        "#,
      0,
  }
}
