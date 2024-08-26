use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_expression.push(walk_object_expression);
  },
  compat {
    name: "trailing_commas_trailing_commas_in_object_literals",
    description: "对象字面量中的尾随逗号",
    tags: [],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "9.5",
      opera_android: "10.1",
      safari: "3",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_object_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectExpression| {
    it.trailing_comma.is_some()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "trailing_commas_trailing_commas_in_object_literals",
    setup,

    should_ok_when_object_expression,
    r#"
      const obj = {
        prop1: 'value1',
        prop2: 'value2',
        prop3: 'value3',
      };
    "#,
    1,

    should_ok_when_object_expression_then,
    r#"
      const obj = {
        prop1: 'value1',
        prop2: 'value2',
        prop3: 'value3',
      };

      const obj2 = {
        prop1: 'value1',
        prop2: 'value2',
        prop3: 'value3',
      };
    "#,
    2,
  }
}
