use std::marker::PhantomData;

use oxc_ast::{
  ast::{MethodDefinitionKind, PropertyKind},
  visit::walk,
  AstKind, Visit,
};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators::common_trait::CommonTrait,
};

pub struct FunctionsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  compat: Compat,
}

impl CommonTrait for FunctionsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> FunctionsVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let compat: Compat =
      from_str(include_str!("./get_computed_property_names.json")).unwrap();
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

impl<'a> Visit<'a> for FunctionsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_syntax::scope::ScopeFlags,
  ) {
    let code_seq = self.get_source_code(it.span).to_string();
    if let Some(parent) = self.parent_stack.last() {
      let is_get_in_computed = match parent {
        AstKind::ObjectProperty(parent) => {
          PropertyKind::Get == parent.kind && parent.computed
        }
        AstKind::MethodDefinition(parent) => {
          MethodDefinitionKind::Get == parent.kind && parent.computed
        }
        _ => false,
      };

      if is_get_in_computed {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seq.clone(),
          compat: self.compat.clone(),
        });
      }
    }
    walk::walk_function(self, it, flags);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_get_in_computed_property_name() {
    let source_code = r##"
const expr = "foo";

const obj = {
  get [expr]() {
    return "bar";
  },
};
"##;
    let allocator = Allocator::default();
    t_any(
      "get_computed_property_names",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }

  #[test]
  fn should_exits_get_in_class_computed_property_name() {
    let source_code = r##"
const expr = "foo";

class MyConstants {
  static get [expr]() {
    return "foo";
  }
}
"##;
    let allocator = Allocator::default();
    t_any(
      "get_computed_property_names",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }
}
