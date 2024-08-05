use std::marker::PhantomData;

use oxc_ast::{
  ast::{BindingPattern, BindingPatternKind, Expression},
  AstKind, Visit,
};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators_n::common_trait::CommonTrait,
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
      from_str(include_str!("./generator_methods_not_constructable.json"))
        .unwrap();
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
    let code_seg = self.get_source_code(it.span).to_string();

    if let Some(parent) = self.parent_stack.last() {
      let is_generator = match parent {
        AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => {
          !it.r#async && it.generator
        }
        _ => false,
      };
      if is_generator {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seg.clone(),
          compat: self.compat.clone(),
        });
      }
    }

    oxc_ast::visit::walk::walk_function(self, it, flags);
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
const obj = {
  g: function* () {
    let index = 0;
    while (true) {
      yield index++;
    }
  },
};    
"##;
    let allocator = Allocator::default();
    t_any(
      "generator_methods_not_constructable",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }
}
