use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_formal_parameters.push(walk_formal_parameters);
  },
  compat {
    name: "rest_parameters",
    description: "剩余参数",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "47",
      chrome_android: "47",
      firefox: "15",
      firefox_android: "15",
      opera: "47",
      opera_android: "47",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      oculus: "47",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_formal_parameters,
  |ctx: &mut Context, it: &oxc_ast::ast::FormalParameters| {
    it.rest.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "rest_parameters",
    setup,

    should_ok_when_use_rest_parameters,
    r#"
      function sum(a,b,...theArgs) {
        let total = 0;
        for (const arg of theArgs) {
          total += arg;
        }
        return total;
      }
    "#,
    1,

    should_ok_when_not_use_rest_parameters,
    r#"
      function sum(a,b) {
        return a + b;
      }
    "#,
    0,

    should_ok_when_not_use_rest_parameters_with_no_parameters,
    r#"
      function sum() {
        return 0;
      }
    "#,
    0,
  }
}
