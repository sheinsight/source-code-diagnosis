use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators::common_trait::CommonTrait,
};

pub struct HashbangCommentsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for HashbangCommentsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> HashbangCommentsVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./hashbang_comments.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      compat: compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for HashbangCommentsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_program(&mut self, it: &oxc_ast::ast::Program<'a>) {
    // TODO hashbang why is none ?
    println!("visit_program-------> {:?}", it.hashbang);
    // if it.hashbang.is_some() {
    //   self.cache.push(CompatBox {
    //     start: it.span.start,
    //     end: it.span.end,
    //     code_seg: self.get_source_code(it.span).to_string(),
    //     compat: self.compat.clone(),
    //   });
    // }
    walk::walk_program(self, it);
  }
}

#[cfg(test)]
mod tests {

  use oxc_allocator::Allocator;

  use crate::syntax::operators::t::t_any;

  use super::*;

  //   #[test]
  //   fn should_exits_hashbang_comments() {
  //     let source_code = r##"

  // #!/usr/bin/env node

  // console.log("Hello world");

  // "##;
  //     let allocator = Allocator::default();
  //     t_any(
  //       "hashbang_comments",
  //       source_code,
  //       &allocator,
  //       HashbangCommentsVisitor::new,
  //     );
  //   }
}
