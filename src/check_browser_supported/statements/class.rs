use crate::create_compat;

create_compat! {

  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_class.push(walk_class);
  },
  compat {
    name: "class",
    description: "The class declaration creates a binding of a new class to a given name.",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "45",
      firefox_android: "45",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "13",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_class,
  |ctx: &mut Context, it: &oxc_ast::ast::Class| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "class",
    setup,

    should_ok_when_class_declaration,
    r#"
    class A {}
    "#,
    1,
  }
}
