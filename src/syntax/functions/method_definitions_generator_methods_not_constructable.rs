use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
    setup,
    |v: &mut SyntaxVisitor| {
        v.walk_function.push(walk_function);
    },
    compat {
        name: "method_definitions_generator_methods",
        description: "generator methods",
        tags: [
            "web-features:generator-methods",
            "web-features:snapshot:ecmascript-2015"
        ],
        support: {
            chrome: "42",
            chrome_android: "42",
            firefox: "43",
            firefox_android: "43",
            opera: "42",
            opera_android: "42",
            safari: "9.1",
            safari_ios: "9.1",
            edge: "13",
            oculus: "42",
            node: "6.0.0",
            deno: "1.0",
        }
    },
    walk_function,
    |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &oxc_semantic::ScopeFlags, _is_strict_mode: bool| {
        if let Some(parent) = ctx.stack.last() {
            match parent {
                AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => {
                    !it.r#async && it.generator
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
      "method_definitions_generator_methods",
      setup,

      should_ok_when_use_generator_methods,
      r#"
          const obj = {
            *f() {
              yield 1;
              yield 2;
              yield 3;
            },
          };
        "#,
      1,

      should_ok_when_not_use_generator_methods,
      r#"
          const obj = {
            f() {
              return 1;
            },
          };
        "#,
      0,

      should_ok_when_use_generator_methods_with_computed_property,
      r#"
          const obj = {
            *[expr]() {
              yield 1;
              yield 2;
              yield 3;
            },
          };
        "#,
      1,

      should_ok_when_use_async_generator_methods,
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
