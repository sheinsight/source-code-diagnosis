use oxc::ast::{ast::Expression, AstKind};

use crate::create_compat;

create_compat! {
  MethodDefinitionsAsyncGeneratorMethods,
  compat {
    name: "method_definitions.async_generator_methods",
    description: "async methods",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "55",
      chrome_android: "55",
      firefox: "52",
      firefox_android: "52",
      safari: "10",
      safari_ios: "10",
      edge: "15",
      node: "7.6.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::ObjectProperty(prop) = node.kind() {
        if prop.method {
            if let Expression::FunctionExpression(func) = &prop.value {
                return func.r#async && func.generator;
            }
        }
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::MethodDefinitionsAsyncGeneratorMethods;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: MethodDefinitionsAsyncGeneratorMethods::default(),
      source_code: r#"
         const obj = {
            async *f() { yield 1; },
          };
      "#,
      eq: [
        r#"async *f() { yield 1; }"#,
      ],
      ne: []
    },

    should_ok_when_use_method_definitions_with_computed_property:{
      setup: MethodDefinitionsAsyncGeneratorMethods::default(),
      source_code: r#"
        const obj = {
          *f() { yield 1; },
        };
      "#,
      eq: [
      ],
      ne: []
    },

    should_ok_when_use_async_methods_with_generator:{
      setup: MethodDefinitionsAsyncGeneratorMethods::default(),
      source_code: r#"
        const obj = {
          async *[expr]() { yield 1; },
        };
      "#,
      eq: [
        r#"async *[expr]() { yield 1; }"#,
      ],
      ne: []
    }

  }
}
