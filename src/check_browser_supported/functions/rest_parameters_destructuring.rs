use oxc_ast::ast::BindingPatternKind;

use crate::create_compat_2;

create_compat_2! {
  RestParametersDestructuring,
  compat {
    name: "rest_parameters",
    description: "Rest parameters",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/rest_parameters",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "47.0.0",
      chrome_android: "47.0.0",
      firefox: "15.0.0",
      firefox_android: "15.0.0",
      safari: "10.0.0",
      safari_ios: "10.0.0",
      edge: "12.0.0",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::FormalParameters(params) = node.kind() {
        if let Some(rest) = &params.rest {
            return matches!(rest.argument.kind, BindingPatternKind::ArrayPattern(_))
        }
    }
    false
  }
}

#[cfg(test)]
mod tests {

  use super::RestParametersDestructuring;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: RestParametersDestructuring::default(),
      source_code: r#"
        function ignoreFirst(...[, b, c]) {
          return b + c;
        }
      "#,
      eq: [
        r#"(...[, b, c])"#,
      ],
      ne: []
    },

    should_ok_when_not_use_rest_parameters_destructuring:{
      setup: RestParametersDestructuring::default(),
      source_code: r#"
        function ignoreFirst(...rest) {
          return rest;
        }
      "#,
      eq: [
      ],
      ne: []
    }
  }
}
