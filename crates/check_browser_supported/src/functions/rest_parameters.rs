use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  RestParameters,
  compat {
    name: "rest_parameters",
    description: "Rest parameters",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/rest_parameters",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "47",
      chrome_android: "47",
      firefox: "15",
      firefox_android: "15",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    matches!(node.kind(), AstKind::FormalParameters(params) if params.rest.is_some())

  }
}

#[cfg(test)]
mod tests {

  use super::RestParameters;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: RestParameters::default(),
      source_code: r#"
        function sum(a,b,...theArgs) {
          let total = 0;
          for (const arg of theArgs) {
            total += arg;
          }
          return total;
        }
      "#,
      eq: [
        r#"(a,b,...theArgs)"#,
      ],
      ne: []
    },

  }
}
