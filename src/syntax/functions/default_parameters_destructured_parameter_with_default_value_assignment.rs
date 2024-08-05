use std::marker::PhantomData;

use oxc_ast::{
  ast::{BindingPattern, BindingPatternKind},
  visit::walk,
  AstKind, Visit,
};
use oxc_span::Span;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators_n::common_trait::CommonTrait,
};

fn has_assignment_pattern(binding_pattern: &BindingPattern) -> bool {
  struct Checker {
    found: bool,
  }

  impl<'a> Visit<'a> for Checker {
    fn visit_assignment_pattern(
      &mut self,
      _pattern: &oxc_ast::ast::AssignmentPattern<'a>,
    ) {
      self.found = true;
    }
  }

  let mut checker = Checker { found: false };
  checker.visit_binding_pattern(binding_pattern);
  checker.found
}

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
    let compat: Compat = from_str(include_str!("./default_parameters_destructured_parameter_with_default_value_assignment.json")).unwrap();
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

impl<'a> Visit<'a> for FunctionsVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_formal_parameter(&mut self, it: &oxc_ast::ast::FormalParameter<'a>) {
    let code_seg = self.get_source_code(it.span).to_string();

    if let BindingPatternKind::AssignmentPattern(pattern) = &it.pattern.kind {
      if has_assignment_pattern(&pattern.left) {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seg.clone(),
          compat: self.compat.clone(),
        });
      }
    }
    walk::walk_formal_parameter(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_test() {
    let source_code = r##"
function preFilledArray([x = 1, y] = []) {
  return x + y;
}


function preFilledArray1({name="李四",age=12} = {}) {
  return x + y;
}    
"##;
    let allocator = Allocator::default();
    t_any(
      "default_parameters_destructured_parameter_with_default_value_assignment",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }
}
