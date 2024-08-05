use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

pub struct DestructuringVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for DestructuringVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> DestructuringVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./destructuring_rest_in_objects.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      compat,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for DestructuringVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_pattern(&mut self, pat: &oxc_ast::ast::ObjectPattern<'a>) {
    if let Some(_) = pat.rest {
      self.cache.push(CompatBox {
        start: pat.span.start,
        end: pat.span.end,
        code_seg: self.get_source_code(pat.span).to_string(),
        compat: self.compat.clone(),
      });
    }

    walk::walk_object_pattern(self, pat);
  }
}

#[cfg(test)]
mod tests {
  use oxc_allocator::Allocator;

  use crate::syntax::operators::t::{t, t_any};

  use super::*;

  #[test]
  fn should_exist_rest_in_objects_of_rest_in_objects() {
    let source_code = r##"
  const {a, ...b} = object;
        "##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_objects",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_objects_of_for_of() {
    let source_code = r##"
const people = [];
for (const {
  name: n,
  family: { father: f,...rest },

} of people) {
  console.log(`Name: ${n}, Father: ${f}`);
}
"##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_objects",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }
}
