use oxc_ast::ast::PrivateInExpression;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_private_in_expression.push(walk_private_in_expression);
  },
  compat {
    name: "classes_private_class_fields_in",
    description: "private class fields 'in' operator",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2022"
    ],
    support: {
      chrome: "91",
      chrome_android: "91",
      firefox: "90",
      firefox_android: "90",
      safari: "15",
      safari_ios: "15",
      edge: "91",
      node: "16.4.0",
      deno: "1.9",
    }
  },
  walk_private_in_expression,
  |ctx: &mut Context, it: &PrivateInExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_private_class_fields_in",
    setup,

    should_ok_when_use_private_in_expression,
    r#"
      class C {
        #x;
        constructor(x) {
          this.#x = x;
        }
        static getX(obj) {
          if (#x in obj) return obj.#x;
          return "obj must be an instance of C";
        }
      }
    "#,
    1,

    should_ok_when_not_use_private_in_expression,
    r#""#,
    0,
  }
}
