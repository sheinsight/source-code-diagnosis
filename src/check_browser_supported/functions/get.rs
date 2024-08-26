use oxc_ast::{
  ast::{MethodDefinitionKind, PropertyKind},
  AstKind,
};

use crate::create_compat;

create_compat! {
    setup,
    |v: &mut SyntaxVisitor| {
        v.walk_function.push(walk_function);
    },
    compat {
        name: "get",
        description: "getter 方法",
        tags: ["web-features:snapshot:ecmascript-5"],
        support: {
            chrome: "1",
            chrome_android: "1",
            firefox: "1.5",
            firefox_android: "1.5",
            opera: "9.5",
            opera_android: "9.5",
            safari: "3",
            safari_ios: "1",
            edge: "12",
            oculus: "1",
            node: "0.10.0",
            deno: "1.0",
        }
    },
    walk_function,
    |ctx: &mut Context, it: &oxc_ast::ast::Function, flags: &oxc_semantic::ScopeFlags| {
        if let Some(parent) = ctx.stack.last() {
            match parent {
                AstKind::ObjectProperty(parent) => PropertyKind::Get == parent.kind,
                AstKind::MethodDefinition(parent) => MethodDefinitionKind::Get == parent.kind,
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
      "get",
      setup,

      should_ok_when_use_get,
      r#"
          const obj = {
            get latest() {
              return this.log[this.log.length - 1];
            },
          };
        "#,
      1,

      should_ok_when_not_use_get,
      r#"
          const obj = {
            latest() {
              return this.log[this.log.length - 1];
            },
          };
        "#,
      0,

      should_ok_when_use_get_with_computed_property,
      r#"
          const obj = {
            get [expr]() {
              return "bar";
            },
          };
        "#,
      1,
  }
}
