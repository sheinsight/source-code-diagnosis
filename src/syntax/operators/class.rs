use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_class.push(walk_class);
  },
  compat {
    name: "operators_class",
    description: "class 关键字",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "42",
      chrome_android: "42",
      firefox: "45",
      firefox_android: "45",
      opera: "42",
      opera_android: "42",
      safari: "7",
      safari_ios: "7",
      edge: "13",
      oculus: "42",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_class,
  |ctx: &mut Context, it: &oxc_ast::ast::Class| {
    it.is_expression()
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_class",
    setup,

    should_ok_when_use_class,
    r#"
      const Rectangle = class {
        constructor(height, width) {
          this.height = height;
          this.width = width;
        }
      };
    "#,
    1
  }
}
