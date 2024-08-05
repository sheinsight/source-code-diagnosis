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
      from_str(include_str!("./destructuring_rest_in_arrays.json")).unwrap();
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

  fn is_destructuring(&self) -> bool {
    match self.parent_stack.last() {
      Some(AstKind::VariableDeclarator(_))
      | Some(AstKind::AssignmentTargetPattern(_)) => true,
      _ => false,
    }
  }
}

impl<'a> Visit<'a> for DestructuringVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_array_pattern(&mut self, it: &oxc_ast::ast::ArrayPattern<'a>) {
    if let Some(_) = it.rest {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.compat.clone(),
      });
    }

    walk::walk_array_pattern(self, it);
  }

  fn visit_array_assignment_target(
    &mut self,
    it: &oxc_ast::ast::ArrayAssignmentTarget<'a>,
  ) {
    if let Some(_) = it.rest {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.compat.clone(),
      });
    }
    walk::walk_array_assignment_target(self, it);
  }
}

#[cfg(test)]
mod tests {
  use oxc_allocator::Allocator;

  use crate::syntax::operators::t::{t, t_any};

  use super::*;

  #[test]
  fn should_exist_rest_in_arrays_of_rest_in_arrays() {
    let source_code = r##"
    const [a, ...b] = array;
        "##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_arrays",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_arrays_of_only_rest_in_arrays() {
    let source_code = r##"
const [...b] = array;
"##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_arrays",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }
}
