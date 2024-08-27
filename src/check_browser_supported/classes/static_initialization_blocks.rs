use oxc_ast::ast::StaticBlock;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_static_block.push(walk_static_block);
  },
  compat {
    name: "classes_static_initialization_blocks",
    description: "static initialization blocks",
    tags: [
      "web-features:class-syntax",
      "web-features:snapshot:ecmascript-2022"
    ],
    support: {
      chrome: "94",
      chrome_android: "94",
      firefox: "93",
      firefox_android: "93",
      safari: "16.4",
      safari_ios: "16.4",
      edge: "94",
      node: "16.11.0",
      deno: "1.14",
    }
  },
  walk_static_block,
  |ctx: &mut Context, it: &StaticBlock| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "classes_static_initialization_blocks",
    setup,

    should_ok_when_use_static_initialization_blocks,
    r#"
      class A {
        static {
          console.log('Static initialization block called');
        }
      }
    "#,
    1,

    should_ok_when_use_two_static_initialization_blocks,
    r#"
      class A {
        static {
          console.log('Static initialization block called');
        }
        static {
          console.log('Static initialization block called');
        }
      }
    "#,
    2,

    should_ok_when_not_use_static_initialization_blocks,
    r#"
      class H{ }
    "#,
    0
  }
}
