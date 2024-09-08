use oxc_ast::{
  ast::{MethodDefinitionKind, PropertyKind},
  AstKind,
};

use crate::create_compat_2;

create_compat_2! {
  GetComputedPropertyNames,
  compat {
    name: "get.computed_property_names",
    description: "Get computed property names",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "46",
      chrome_android: "46",
      firefox: "34",
      firefox_android: "34",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::ObjectProperty(object_property) = node.kind() {
      return matches!(object_property.kind, PropertyKind::Get) && object_property.computed;
    }

    if let AstKind::MethodDefinition(method_definition) = node.kind() {
      return matches!(method_definition.kind, MethodDefinitionKind::Get) && method_definition.computed;
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::GetComputedPropertyNames;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: GetComputedPropertyNames::default(),
      source_code: r#"
        const obj = {
          get [expr]() { return "bar"; },
        };
      "#,
      eq: [
        r#"get [expr]() { return "bar"; }"#,
      ],
      ne: []
    },

    should_ng_when_not_use_get_computed_property_names: {
      setup: GetComputedPropertyNames::default(),
      source_code: r#"
        const obj = {
          get foo() { return "bar"; },
        };
      "#,
      eq: [],
      ne: [
        r#"get foo() { return "bar"; }"#,
      ]
    },

    should_ok_when_use_get_computed_property_names_with_class_declaration: {
      setup: GetComputedPropertyNames::default(),
      source_code: r#"
        class A {
          get [expr]() { return "bar"; }
        }
      "#,
      eq: [
        r#"get [expr]() { return "bar"; }"#,
      ],
      ne: []
    }
  }
}
