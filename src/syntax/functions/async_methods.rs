use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
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
      from_str(include_str!("./async_methods.json")).unwrap();
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
      let is_async = match parent {
        AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => {
          it.r#async && !it.generator
        }
        _ => false,
      };
      if is_async {
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
  use crate::syntax::operators::t::{t_any, t_any_not};
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_async_method_1() {
    let source_code = r##"
const obj = {
  f: async function () {
    await somePromise;
  },
};    
"##;
    let allocator = Allocator::default();
    t_any(
      "async_methods",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }

  #[test]
  fn should_exits_async_method_2() {
    let source_code = r##"
const obj2 = {
  async f() {
    await somePromise;
  },
};
"##;
    let allocator = Allocator::default();
    t_any(
      "async_methods",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }

  #[test]
  fn should_not_exits_async_method() {
    let source_code = r##"
const obj2 = {
  async *f() {
    await somePromise;
  },
};
"##;
    let allocator = Allocator::default();
    t_any_not(
      "async_methods",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }
}
