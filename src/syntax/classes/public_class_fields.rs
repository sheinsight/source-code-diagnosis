use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators_n::common_trait::CommonTrait,
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
    let compat: Compat =
      from_str(include_str!("./public_class_fields.json")).unwrap();
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

  fn visit_property_definition(
    &mut self,
    it: &oxc_ast::ast::PropertyDefinition<'a>,
  ) {
    let code_seq = self.get_source_code(it.span).to_string();

    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seq.clone(),
      compat: self.compat.clone(),
    });
    oxc_ast::visit::walk::walk_property_definition(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_classes_public_class_fields() {
    let source_code = r##"
class C {
  age=12;
}
"##;
    let allocator = Allocator::default();
    t_any(
      "public_class_fields",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }

  #[test]
  fn should_exits_static() {
    let source_code = r##"
class ClassWithStaticMethod {
  staticProperty = 'someValue';
}
"##;
    let allocator = Allocator::default();
    t_any(
      "public_class_fields",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }
}
