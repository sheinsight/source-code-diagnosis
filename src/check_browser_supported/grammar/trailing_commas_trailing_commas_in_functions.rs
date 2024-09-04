use crate::{
  check_browser_supported::compat::get_source_code_segment, create_compat_2,
};

create_compat_2! {
  TrailingCommasInFunctions,
  compat {
    name: "trailing_commas_trailing_commas_in_functions",
    description: "Trailing commas in function parameters",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas#trailing_commas_in_functions",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "58.0.0",
      chrome_android: "58.0.0",
      firefox: "52.0.0",
      firefox_android: "52.0.0",
      safari: "10.0.0",
      safari_ios: "10.0.0",
      edge: "14.0.0",
      node: "8.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    if let AstKind::FormalParameters(params) = node.kind() {
      let source_segment = get_source_code_segment(source_code, params);
      let trimmed = source_segment.trim_end();
      if let Some(last) = trimmed.split(",").map(|item| item.trim()).last() {
        if last == ")" {
          return true;
        }
      }
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::TrailingCommasInFunctions;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_trailing_comma_in_function_declaration:{
      setup: TrailingCommasInFunctions::default(),
      source_code: r#"
        function myFunction(
          param1,
          param2,
        ) {
          // Function body
        }
      "#,
      eq: [
        r#"(
          param1,
          param2,
        )"#,
      ],
      ne: []
    },

    should_ok_when_use_trailing_comma_in_function_expression:{
      setup: TrailingCommasInFunctions::default(),
      source_code: r#"
        const myFunction = function(
          param1,
          param2,
        ) {
          // Function body
        };
      "#,
      eq: [
        r#"(
          param1,
          param2,
        )"#,
      ],
      ne: []
    },

    should_not_ok_when_no_trailing_comma:{
      setup: TrailingCommasInFunctions::default(),
      source_code: r#"
        function myFunction(param1, param2) {
          // Function body
        }
      "#,
      eq: [],
      ne: [
        r#"(param1, param2)"#,
      ]
    },
  }
}
