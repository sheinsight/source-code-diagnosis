use std::sync::OnceLock;

use oxc_ast::ast::StaticBlock;
use serde_json5::from_str;

use crate::syntax::{
  common::Context,
  compat::{Compat, CompatBox},
};

static CONSTRUCTOR_COMPAT: OnceLock<Compat> = OnceLock::new();

pub fn walk_static_block(ctx: &mut Context, it: &StaticBlock) {
  let compat = CONSTRUCTOR_COMPAT.get_or_init(|| {
    from_str(include_str!("./static_initialization_blocks.json")).unwrap()
  });
  ctx
    .usage
    .push(CompatBox::new(it.span.clone(), compat.clone()));
}

#[cfg(test)]
mod tests {
  use crate::{assert_ok_count, syntax::visitor::SyntaxVisitor};

  fn setup_walk_static_block(v: &mut SyntaxVisitor) {
    v.walk_static_block.push(super::walk_static_block);
  }

  assert_ok_count! {
    "classes_static_initialization_blocks",
    setup_walk_static_block,

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
