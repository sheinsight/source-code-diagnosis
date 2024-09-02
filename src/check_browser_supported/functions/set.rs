use oxc_ast::ast::{MethodDefinitionKind, PropertyKind};

use crate::create_compat_2;

create_compat_2! {
  Set,
  compat {
    name: "get",
    description: "Get computed property names",
    mdn_url: "https://developer.mozilla.org/docs/Web/JavaScript/Reference/Functions/get",
    tags: [
      "web-features:snapshot:ecmascript-1"
    ],
    support: {
      chrome: "1.0.0",
      chrome_android: "1.0.0",
      firefox: "1.5.0",
      firefox_android: "1.5.0",
      safari: "3.0.0",
      safari_ios: "1.0.0",
      edge: "12.0.0",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {

    if let AstKind::ObjectProperty(object_property) = node.kind() {
      return matches!(object_property.kind, PropertyKind::Set) ;
    }

    if let AstKind::MethodDefinition(method_definition) = node.kind() {
      return matches!(method_definition.kind, MethodDefinitionKind::Set);
    }

    false
  }
}

#[cfg(test)]
mod tests {

  use super::Set;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_declaration:{
      setup: Set::default(),
      source_code: r#"
        const language = {
          set current(name) { this.log.push(name); },
          log: [],
        };
      "#,
      eq: [
        r#"set current(name) { this.log.push(name); }"#,
      ],
      ne: []
    },

    should_ng_when_not_use_get_computed_property_names1: {
      setup: Set::default(),
      source_code: r#"
          const language = {
            current(name) {
              this.log.push(name);
          },
          log: [],
        };  
      "#,
      eq: [

      ],
      ne: [

      ]
    },

  }
}
