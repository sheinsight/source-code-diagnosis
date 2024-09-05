use crate::{
  check_browser_supported::compat::get_source_code_segment, create_compat_2,
};

create_compat_2! {
  ArrowFunctionsTrailingComma,
  compat {
    name: "functions.arrow_functions.trailing_comma",
    description: "Arrow functions",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Arrow_functions",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "58",
      chrome_android: "58",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "8.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::FormalParameters(params) = node.kind() {
      let source_segment = get_source_code_segment(source_code,params);
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

  use super::ArrowFunctionsTrailingComma;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: ArrowFunctionsTrailingComma::default(),
      source_code: r#"
         const func4 = (a,b,c,) => 1;
      "#,
      eq: [
        r#"(a,b,c,)"#,
      ],
      ne: [
      ]
    },

    should_ok_when_not_use_arrow_functions_trailing_comma:{
      setup: ArrowFunctionsTrailingComma::default(),
      source_code: r#"
         const func4 = (a,b,c) => 1;
      "#,
      eq: [

      ],
      ne: [
      ]
    },


  }
}
