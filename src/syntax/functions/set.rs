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
    name: "set",
    description: "setter 方法",
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
  |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &oxc_semantic::ScopeFlags, _is_strict_mode: bool| {
    if let Some(parent) = ctx.stack.last() {
      match parent {
        AstKind::ObjectProperty(parent) => PropertyKind::Set == parent.kind,
        AstKind::MethodDefinition(parent) => MethodDefinitionKind::Set == parent.kind,
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
    "set",
    setup,

    should_ok_when_use_set,
    r#"
      const language = {
        set current(name) {
          this.log.push(name);
        },
        log: [],
      };
    "#,
    1,

    should_ok_when_not_use_set,
    r#"
      const language = {
        current(name) {
          this.log.push(name);
        },
        log: [],
      };
    "#,
    0,

    should_ok_when_use_set_with_async,
    r#"
      const language = {
        async set current(name) {
          this.log.push(name);
        },
        log: [],
      };
    "#,
    0,
  }
}
