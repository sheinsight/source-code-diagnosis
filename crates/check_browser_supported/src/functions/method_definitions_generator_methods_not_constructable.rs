use oxc::ast::{ast::Expression, AstKind};

use crate::create_compat;

create_compat! {
  MethodDefinitionsGeneratorMethodsNotConstructable,
  compat {
    name: "method_definitions.generator_methods_not_constructable",
    description: "async methods",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "42",
      chrome_android: "42",
      firefox: "43",
      firefox_android: "43",
      safari: "9",
      safari_ios: "9",
      edge: "13",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::ObjectProperty(prop) = node.kind() {
        if prop.method {
            if let Expression::FunctionExpression(func) = &prop.value {
                return !func.r#async && func.generator;
            }
        }
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::MethodDefinitionsGeneratorMethodsNotConstructable;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: MethodDefinitionsGeneratorMethodsNotConstructable::default(),
      source_code: r#"
         const obj = {
            *f() { yield 1; },
          };
      "#,
      eq: [
        r#"*f() { yield 1; }"#,
      ],
      ne: []
    },

    should_ok_when_use_method_definitions_with_computed_property:{
      setup: MethodDefinitionsGeneratorMethodsNotConstructable::default(),
      source_code: r#"
        const obj = {
            *[expr]() { yield 1; },
          };
      "#,
      eq: [
        r#"*[expr]() { yield 1; }"#,
      ],
      ne: []
    },

  }
}
