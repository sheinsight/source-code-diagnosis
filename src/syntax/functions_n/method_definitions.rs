use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_span::Span;
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
      from_str(include_str!("./method_definitions.json")).unwrap();
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
      let is_define = match parent {
        AstKind::ObjectProperty(_) | AstKind::MethodDefinition(_) => true,
        _ => false,
      };
      if is_define {
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
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_method_definitions() {
    let source_code = r##"
const obj = {
  foo() {
    return 'bar';
  },
  bar: function () { },
};    
"##;
    let allocator = Allocator::default();
    t_any(
      "method_definitions",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }

  #[test]
  fn should_exits_method_definitions_in_class() {
    let source_code = r##"
class ClassWithPublicInstanceMethod {
  publicMethod() {
    return "hello world";
  }
  secondPublicMethod() {
    return "goodbye world";
  }
}
"##;
    let allocator = Allocator::default();
    t_any(
      "method_definitions",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }
}
