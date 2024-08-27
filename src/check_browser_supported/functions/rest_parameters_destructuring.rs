use oxc_ast::ast::BindingPatternKind;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_formal_parameters.push(walk_formal_parameters);
  },
  compat {
    name: "rest_parameters_destructuring",
    description: "解构剩余参数",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "49",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_formal_parameters,
  |ctx: &mut Context, it: &oxc_ast::ast::FormalParameters| {
    if let Some(rest) = &it.rest {
      matches!(rest.argument.kind, BindingPatternKind::ArrayPattern(_))
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
    "rest_parameters_destructuring",
    setup,

    should_ok_when_use_rest_parameters_destructuring,
    r#"
      function ignoreFirst(...[, b, c]) {
        return b + c;
      }
    "#,
    1,

    should_ok_when_not_use_rest_parameters_destructuring,
    r#"
      function ignoreFirst(...rest) {
        return rest;
      }
    "#,
    0,

    should_ok_when_not_use_rest_parameters_destructuring_with_no_parameters,
    r#"
      function ignoreFirst() {
        return 'hello';
      }
    "#,
    0,
  }
}
