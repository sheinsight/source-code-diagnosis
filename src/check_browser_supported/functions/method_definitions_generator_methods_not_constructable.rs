use oxc_ast::ast::Expression;

use crate::create_compat_2;

create_compat_2! {
  MethodDefinitionsGeneratorMethodsNotConstructable,
  compat {
    name: "method_definitions_async_generator_methods",
    description: "async methods",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/Method_definitions",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "42.0.0",
      chrome_android: "42.0.0",
      firefox: "43.0.0",
      firefox_android: "43.0.0",
      safari: "9.1.0",
      safari_ios: "9.1.0",
      edge: "13.0.0",
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
