use std::marker::PhantomData;

use oxc_ast::{
  ast::{ClassElement, MethodDefinitionKind},
  AstKind, Visit,
};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators::common_trait::CommonTrait,
};

pub struct ClassesVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for ClassesVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ClassesVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat = from_str(include_str!("./constructor.json")).unwrap();
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

impl<'a> Visit<'a> for ClassesVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_class_body(&mut self, it: &oxc_ast::ast::ClassBody<'a>) {
    for item in &it.body {
      if let ClassElement::MethodDefinition(ele) = item {
        if ele.kind == MethodDefinitionKind::Constructor {
          let code_seq = self.get_source_code(ele.span).to_string();
          self.cache.push(CompatBox {
            start: ele.span.start,
            end: ele.span.end,
            code_seg: code_seq,
            compat: self.compat.clone(),
          });
        }
      }
    }
    oxc_ast::visit::walk::walk_class_body(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_classes_constructor() {
    let source_code = r##"
class Rectangle {
  constructor(){}
}    
"##;
    let allocator = Allocator::default();
    t_any("constructor", source_code, &allocator, ClassesVisitor::new);
  }
}
