use oxc_ast::{
  ast::{Function, MethodDefinitionKind, PropertyKind},
  AstKind,
};

use crate::create_compat;

create_compat! {
    setup,
    |v: &mut SyntaxVisitor| {
        v.walk_function.push(walk_function);
    },
    compat {
        name: "get_computed_property_names",
        description: "generator functions",
        tags: [
            "web-features:generator-functions",
            "web-features:snapshot:ecmascript-2015"
        ],
        support: {
            chrome: "39",
            chrome_android: "39",
            firefox: "26",
            firefox_android: "26",
            opera: "26",
            opera_android: "26",
            safari: "10",
            safari_ios: "10",
            edge: "13",
            oculus: "5.0",
            node: "4.0.0",
            deno: "1.0",
        }
    },
    walk_function,
    |ctx: &mut Context, it: &Function, _flags: &oxc_semantic::ScopeFlags| {
      if let Some(parent) = ctx.stack.last() {
        let is_get_in_computed = match parent {
          AstKind::ObjectProperty(parent) => {
            PropertyKind::Get == parent.kind && parent.computed
          }
          AstKind::MethodDefinition(parent) => {
            MethodDefinitionKind::Get == parent.kind && parent.computed
          }
          _ => false,
        };
        is_get_in_computed
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
    "get_computed_property_names",
    setup,

    should_ok_when_use_get_computed_property_names,
    r#"
      const obj = {
        get [expr]() {
          return "bar";
        },
      };
    "#,
    1,

    should_ok_when_not_use_get_computed_property_names,
    r#"
      const obj = {
        get foo() {
          return "bar";
        },
      };
    "#,
    0


  }
}
