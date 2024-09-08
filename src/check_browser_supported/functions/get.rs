use oxc_ast::{
  ast::{MethodDefinitionKind, PropertyKind},
  AstKind,
};

use crate::create_compat_2;

create_compat_2! {
  Get,
  compat {
    name: "get",
    description: "Get computed property names",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "3",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::ObjectProperty(object_property) = node.kind() {
      return matches!(object_property.kind, PropertyKind::Get) ;
    }

    if let AstKind::MethodDefinition(method_definition) = node.kind() {
      return matches!(method_definition.kind, MethodDefinitionKind::Get);
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::Get;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: Get::default(),
      source_code: r#"
        const obj = {
            get latest() { return this.log[this.log.length - 1]; }
          };
      "#,
      eq: [
        r#"get latest() { return this.log[this.log.length - 1]; }"#,
      ],
      ne: []
    },

    should_ng_when_not_use_get_computed_property_names1: {
      setup: Get::default(),
      source_code: r#"
          const obj = {
            expr() { return "bar"; }
          };
      "#,
      eq: [

      ],
      ne: [

      ]
    },

    should_ng_when_not_use_get_computed_property_names: {
      setup: Get::default(),
      source_code: r#"
          const obj = {
            get [expr]() { return "bar"; }
          };
      "#,
      eq: [
        r#"get [expr]() { return "bar"; }"#,
      ],
      ne: []
    }
  }
}
