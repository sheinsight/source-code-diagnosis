use std::sync::OnceLock;

use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
  visitor::SyntaxVisitor,
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

fn walk_object_property(ctx: &mut Context, it: &oxc_ast::ast::ObjectProperty) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./shorthand_object_literals.json")).unwrap()
  });
  if it.shorthand {
    ctx
      .usage
      .push(CompatBox::new(it.span.clone(), compat.clone()));
  }
}

pub fn setup_shorthand_object_literals(v: &mut SyntaxVisitor) {
  v.walk_object_property.push(walk_object_property);
}

#[cfg(test)]
mod tests {
  use crate::{
    assert_ok_count,
    syntax::grammar::shorthand_object_literals::setup_shorthand_object_literals,
  };

  assert_ok_count! {
    "shorthand_object_literals",
    setup_shorthand_object_literals,

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
