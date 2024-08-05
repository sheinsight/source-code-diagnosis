use std::marker::PhantomData;

use oxc_ast::{
  ast::BindingPatternKind,
  visit::walk::{self},
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
    let compat: Compat = from_str(include_str!("./default_parameters_parameters_without_defaults_after_default_parameters.json")).unwrap();
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

  fn visit_formal_parameters(
    &mut self,
    it: &oxc_ast::ast::FormalParameters<'a>,
  ) {
    let mut flag: i32 = 0;
    let code_seg = self.get_source_code(it.span).to_string();
    for item in &it.items {
      match item.pattern.kind {
        BindingPatternKind::AssignmentPattern(_) => {
          flag = 1;
        }
        BindingPatternKind::BindingIdentifier(_)
        | BindingPatternKind::ObjectPattern(_)
        | BindingPatternKind::ArrayPattern(_) => {
          if flag == 1 {
            flag = -1;
          } else {
            flag = 0;
          }
        }
      }

      if flag == -1 {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seg.clone(),
          compat: self.compat.clone(),
        });
      }
    }
    walk::walk_formal_parameters(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_default_parameters_parameters_without_defaults_after_default_parameters(
  ) {
    let sources = vec![
      r##"function example(a = 1, b) {}"##,
      r##"function example(x,a = 1, b) {}"##,
      r##"function example(x,a = {}, b) {}"##,
    ];

    let allocator = Allocator::default();

    sources.iter().for_each(|item| {
      t_any(
        "default_parameters_parameters_without_defaults_after_default_parameters",
        item,
        &allocator,
        FunctionsVisitor::new,
      );
    });
  }
}
