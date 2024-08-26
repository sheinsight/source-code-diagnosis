use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_new_expression.push(walk_new_expression);
  },
  compat {
    name: "operators_new",
    description: "The new operator creates an instance of a user-defined object type or of one of the built-in object types that has a constructor function.",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_new_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::NewExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_new",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    function Car(make, model, year) {
      this.make = make;
      this.model = model;
      this.year = year;
    }

    const car1 = new Car('Eagle', 'Talon TSi', 1993);
    "#,
    1,
  }
}
