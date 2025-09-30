use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  ArrowFunctions,
  compat {
    name: "functions.arrow_functions",
    description: "Arrow functions",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Arrow_functions",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "45",
      chrome_android: "45",
      firefox: "22",
      firefox_android: "22",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(node.kind(), AstKind::ArrowFunctionExpression(_))
  }
}

#[cfg(test)]
mod tests {

  use super::ArrowFunctions;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: ArrowFunctions::default(),
      source_code: r#"
        const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

        console.log(materials.map((material) => material.length));
      "#,
      eq: [
        r#"(material) => material.length"#,
      ],
      ne: [
      ]
    },

    should_fail_when_not_arrow_function:{
      setup: ArrowFunctions::default(),
      source_code: r#"
        function add(a, b) {
          return a + b;
        }
      "#,
      eq: [
      ],
      ne: [
        r#"function add(a, b)"#,
      ]
    },

    should_ok_with_nested_arrow_function:{
      setup: ArrowFunctions::default(),
      source_code: r#"
        const add = (a, b) => { const sum = (x, y) => x + y; return sum(a, b); };
      "#,
      eq: [
        r#"(a, b) => { const sum = (x, y) => x + y; return sum(a, b); }"#,
        r#"(x, y) => x + y"#,
      ],
      ne: [
      ]
    }
  }
}
