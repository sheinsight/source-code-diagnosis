use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_property.push(walk_object_property);
  },
  compat {
    name: "shorthand_object_literals",
    description: "对象字面量的简写表示法",
    tags: [
      "web-features:snapshot:ecmascript-2015"
    ],
    support: {
      chrome: "43",
      chrome_android: "43",
      firefox: "33",
      firefox_android: "33",
      opera: "43",
      opera_android: "43",
      safari: "9",
      safari_ios: "9",
      edge: "12",
      oculus: "43",
      node: "4.0.0",
      deno: "1.0",
    }
  },
  walk_object_property,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectProperty| {
    it.shorthand
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "shorthand_object_literals",
    setup,

    should_ok_when_use_shorthand_object_literals,
    r#"
      const name = "Alice";
      const age = 30;

      const person = {
        name,
        age,
        sayHello() {
          console.log(`Hello, I'm ${this.name}`);
        },
        [`status_${Date.now()}`]: "active"
      };
    "#,
    2,
  }
}
