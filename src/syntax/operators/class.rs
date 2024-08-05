use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

pub struct ClassVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for ClassVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ClassVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat = from_str(include_str!("./class.json")).unwrap();
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

impl<'a> Visit<'a> for ClassVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_class(&mut self, it: &oxc_ast::ast::Class<'a>) {
    let code_seg = self.get_source_code(it.span).to_string();
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seg,
      compat: self.compat.clone(),
    });
    oxc_ast::visit::walk::walk_class(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_class_when_assignment_expression() {
    let source_code = r##"
const Rectangle = class {
  constructor(height, width) {
    this.height = height;
    this.width = width;
  }
  area() {
    return this.height * this.width;
  }
};
console.log(new Rectangle(5, 8).area());    
"##;
    let allocator = Allocator::default();
    t_any("class", source_code, &allocator, ClassVisitor::new);
  }

  #[test]
  fn should_exits_class_when_declaration() {
    let source_code = r##"
class Rectangle {
  constructor(height, width) {
    this.height = height;
    this.width = width;
  }
}
"##;
    let allocator = Allocator::default();
    t_any("class", source_code, &allocator, ClassVisitor::new);
  }
}
