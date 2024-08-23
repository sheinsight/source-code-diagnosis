use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_function.push(walk_function);
  },
  compat {
    name: "method_definitions",
    description: "方法定义",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "39",
      chrome_android: "39",
      firefox: "34",
      firefox_android: "34",
      opera: "39",
      opera_android: "39",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      oculus: "39",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &oxc_semantic::ScopeFlags, _is_strict_mode: bool| {
    if let Some(parent) = ctx.stack.last() {
      matches!(parent, AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_))
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
    "method_definitions",
    setup,

    should_ok_when_use_method_definitions,
    r#"
      const obj = {
        foo() {
          return 'bar';
        },
        bar: function () { },
      };
    "#,
    2,

    should_ok_when_not_use_method_definitions,
    r#"
      const obj = {
        foo: 'bar',
        bar: 'foo',
      };
    "#,
    0,

    should_ok_when_use_method_definitions_with_computed_property,
    r#"
      const obj = {
        [expr]() {
          return 'bar';
        },
      };
    "#,
    1,

    should_ok_when_use_method_definitions_with_async,
    r#"
      const obj = {
        async foo() {
          return 'bar';
        },
        async bar() { },
      };
    "#,
    2,
  }
}
