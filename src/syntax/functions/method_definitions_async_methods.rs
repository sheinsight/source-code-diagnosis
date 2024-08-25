use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
    setup,
    |v: &mut SyntaxVisitor| {
        v.walk_function.push(walk_function);
    },
    compat {
        name: "method_definitions_async_methods",
        description: "async methods",
        tags: [
            "web-features:async-methods",
            "web-features:snapshot:ecmascript-2017"
        ],
        support: {
            chrome: "55",
            chrome_android: "55",
            firefox: "52",
            firefox_android: "52",
            opera: "42",
            opera_android: "42",
            safari: "10.1",
            safari_ios: "10.3",
            edge: "15",
            oculus: "5.0",
            node: "7.6.0",
            deno: "1.0",
        }
    },
    walk_function,
    |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &oxc_semantic::ScopeFlags| {
        if let Some(parent) = ctx.stack.last() {
            match parent {
                AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => {
                    it.r#async && !it.generator
                }
                _ => false,
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
      "method_definitions_async_methods",
      setup,

      should_ok_when_use_async_methods,
      r#"
          const obj = {
            async f() {
              await somePromise;
            },
          };
        "#,
      1,

      should_ok_when_not_use_async_methods,
      r#"
          const obj = {
            f() {
              await somePromise;
            },
          };
        "#,
      0,

      should_ok_when_use_async_methods_with_computed_property,
      r#"
          const obj = {
            async [expr]() {
              await somePromise;
            },
          };
        "#,
      1,

      should_ok_when_use_async_methods_with_generator,
      r#"
          const obj = {
            async *f() {
              yield 1;
              yield 2;
              yield 3;
            },
          };
        "#,
      0,
  }
}
