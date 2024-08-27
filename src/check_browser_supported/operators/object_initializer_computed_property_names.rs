use oxc_ast::ast::Expression;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_assignment_expression.push(walk_assignment_expression);
      v.walk_variable_declaration.push(walk_variable_declaration);
  },
  compat {
    name: "operators_object_initializer_computed_property_names",
    description: "Computed property names",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "47",
      chrome_android: "47",
      firefox: "34",
      firefox_android: "34",
      safari: "8",
      safari_ios: "8",
      edge: "12",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    if let Expression::ObjectExpression(oe) = &it.right {
      oe.properties.iter().any(|p| {
        matches!(p, oxc_ast::ast::ObjectPropertyKind::ObjectProperty(op) if op.computed)
      })
    } else {
      false
    }
  },
  walk_variable_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::VariableDeclaration| {
    it.declarations.iter().any(|d| {
      if let Some(init) = &d.init {
        if let Expression::ObjectExpression(oe) = init {
          oe.properties.iter().any(|p| {
            matches!(p, oxc_ast::ast::ObjectPropertyKind::ObjectProperty(op) if op.computed)
          })
        } else {
          false
        }
      } else {
        false
      }
    })
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_object_initializer_computed_property_names",
    setup,

    should_detect_computed_property_in_object_initializer,
    r#"
    const propertyName = 'age';
    const person = {
      [propertyName]: 30
    };
    console.log(person.age);
    "#,
    1,

    should_detect_computed_property_in_object_initializer2,
    r#"
    const propertyName = 'age';
    let person;
    person = {
      [propertyName]: 30
    };
    console.log(person.age);
    "#,
    1,

    should_detect_multiple_computed_properties,
    r#"
    const key1 = 'name';
    const key2 = 'age';
    const person = {
      [key1]: 'John',
      [key2]: 30
    };
    "#,
    1,

    should_detect_computed_property_with_expression,
    r#"
    const prefix = 'user';
    const user = {
      [prefix + 'Name']: 'John',
      [prefix + 'Age']: 30
    };
    "#,
    1,

    should_not_detect_regular_object_properties,
    r#"
    const person = {
      name: 'John',
      age: 30
    };
    "#,
    0,

    should_detect_computed_property_in_nested_object,
    r#"
    const key = 'details';
    const user = {
      name: 'John',
      [key]: {
        age: 30,
        city: 'New York'
      }
    };
    "#,
    1,

    should_detect_computed_property_in_object_method,
    r#"
    const methodName = 'greet';
    const person = {
      name: 'John',
      [methodName]() {
        console.log('Hello!');
      }
    };
    "#,
    1,
  }
}
