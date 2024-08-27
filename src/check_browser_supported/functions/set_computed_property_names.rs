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
    name: "set_computed_property_names",
    description: "setter 方法的计算属性名",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "34",
      firefox_android: "34",
      safari: "9.1",
      safari_ios: "9.1",
      edge: "12",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &oxc_semantic::ScopeFlags| {
    if let Some(parent) = ctx.stack.last() {
      let is_set_in_computed = match parent {
        AstKind::ObjectProperty(parent) => {
          PropertyKind::Set == parent.kind && parent.computed
        }
        AstKind::MethodDefinition(parent) => {
          MethodDefinitionKind::Set == parent.kind && parent.computed
        }
        _ => false,
      };

      is_set_in_computed
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
    "set_computed_property_names",
    setup,

    should_ok_when_use_set_computed_property_names,
    r#"
      const obj = {
        set [expr]() {
          return "bar";
        },
      };
    "#,
    1,

    should_ok_when_not_use_set_computed_property_names,
    r#"
      const obj = {
        set foo(value) {
          return "bar";
        },
      };
    "#,
    0,

    should_ok_when_use_set_computed_property_names_with_async,
    r#"
      const obj = {
        async set [expr]() {
          return "bar";
        },
      };
    "#,
    0,
  }
}
