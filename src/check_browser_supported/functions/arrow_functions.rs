use oxc_ast::ast::ArrowFunctionExpression;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_arrow_function_expression.push(arrow_functions);
  },
  compat {
    name: "arrow_functions",
    description: "arrow function expressions",
    tags: [
      "web-features:arrow-functions",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "45",
      chrome_android: "45",
      firefox: "22",
      firefox_android: "22",
      opera: "45",
      opera_android: "45",
      safari: "10",
      safari_ios: "10",
      edge: "12",
      oculus: "45",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  arrow_functions,
  |ctx: &mut Context, _it: &ArrowFunctionExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "arrow_functions",
    setup,

    should_ok_when_use_arrow_functions,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map((material) => material.length));
    "#,
    1,

    should_ok_when_not_use_arrow_functions,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map(function(material) { return material.length; }));
    "#,
    0,

    should_ok_when_use_arrow_functions_with_no_space,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map((material)=>material.length));
    "#,
    1,

    should_ok_when_use_arrow_functions_with_no_parentheses,
    r#"
      const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];

      console.log(materials.map(material => material.length));
    "#,
    1,
  }
}
