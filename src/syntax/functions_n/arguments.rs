use std::marker::PhantomData;

use oxc_ast::{ast::Expression, AstKind, Visit};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators_n::common_trait::CommonTrait,
};

pub struct ArgumentsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for ArgumentsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ArgumentsVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat = from_str(include_str!("./arguments.json")).unwrap();
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

impl<'a> Visit<'a> for ArgumentsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_computed_member_expression(
    &mut self,
    it: &oxc_ast::ast::ComputedMemberExpression<'a>,
  ) {
    let code_seg = self.get_source_code(it.span).to_string();
    if let Expression::ComputedMemberExpression(_) = it.object {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg.clone(),
        compat: self.compat.clone(),
      });
    }

    oxc_ast::visit::walk::walk_computed_member_expression(self, it);
  }

  fn visit_identifier_reference(
    &mut self,
    it: &oxc_ast::ast::IdentifierReference<'a>,
  ) {
    if it.name == "arguments" {
      let code_seg = self.get_source_code(it.span).to_string();
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg.clone(),
        compat: self.compat.clone(),
      });
    }
    oxc_ast::visit::walk::walk_identifier_reference(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_arguments_1() {
    let source_code = r##"
function hello(){
  console.log(arguments);
}
    "##;
    let allocator = Allocator::default();
    t_any("arguments", source_code, &allocator, ArgumentsVisitor::new);
  }

  #[test]
  fn should_exits_arguments_2() {
    let source_code = r##"
function hello(){
  console.log(arguments[0]);
}
    "##;
    let allocator = Allocator::default();
    t_any("arguments", source_code, &allocator, ArgumentsVisitor::new);
  }
}
