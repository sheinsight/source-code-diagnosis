use oxc::ast::AstKind;

use crate::create_compat;

create_compat! {
  ClassesStaticClassFields,
  compat {
    name: "classes.static_class_fields",
    description: "static class fields",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Classes/static",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "45",
      firefox_android: "45",
      safari: "9",
      safari_ios: "9",
      edge: "13",
      node: "6.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str,node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    matches!(
      node.kind(),
      AstKind::PropertyDefinition(property_definition)
      if property_definition.r#static
    ) || matches!(
      node.kind(),
      AstKind::MethodDefinition(method_definition)
      if method_definition.r#static
    )
  }
}

#[cfg(test)]
mod tests {

  use super::ClassesStaticClassFields;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_use_class_static_fields:{
      setup: ClassesStaticClassFields::default(),
      source_code: r#"
        class A { 
          static hello = 12;
          static staticMethod() { }
          static h = () => { }
        }
      "#,
      eq: [
        r#"static hello = 12;"#,
        r#"static staticMethod() { }"#,
        r#"static h = () => { }"#
      ],
      ne: [

      ]
    }
  }
}
