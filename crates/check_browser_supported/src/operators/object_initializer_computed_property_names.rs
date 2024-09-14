use oxc_ast::AstKind;

use crate::create_compat;

create_compat! {
  ObjectInitializerComputedPropertyNames,
  compat {
    name: "operators.object_initializer.computed_property_names",
    description: "计算属性名",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Object_initializer#Computed_property_names",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "47",
      chrome_android: "47",
      firefox: "34",
      firefox_android: "34",
      safari: "8",
      safari_ios: "8",
      edge: "12",
      node: "4.0.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, _source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    match node.kind() {
      AstKind::ObjectProperty(object_property) => object_property.computed,
      AstKind::MethodDefinition(method_definition) => method_definition.computed,
      _ => false,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::ObjectInitializerComputedPropertyNames;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_detect_computed_property_in_object_initializer:{
      setup: ObjectInitializerComputedPropertyNames::default(),
      source_code: r#"
        const propertyName = 'age';
        const person = { [propertyName]: 30 };
        console.log(person.age);
      "#,
      eq: [
        r#"[propertyName]: 30"#,
      ],
      ne: []
    },

    should_detect_computed_property_in_object_method:{
      setup: ObjectInitializerComputedPropertyNames::default(),
      source_code: r#"
        const methodName = 'greet';
        const person = {
          [methodName]() { }
        };
      "#,
      eq: [
        r#"[methodName]() { }"#,
      ],
      ne: []
    },

    should_not_detect_regular_object_properties:{
      setup: ObjectInitializerComputedPropertyNames::default(),
      source_code: r#"
        const person = {
          name: 'John',
          age: 30
        };
      "#,
      eq: [],
      ne: [
        r#"name: 'John',"#,
        r#"age: 30"#,
      ]
    }
  }
}
