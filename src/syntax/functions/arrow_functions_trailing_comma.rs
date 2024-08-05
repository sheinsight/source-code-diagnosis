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
      from_str(include_str!("./arrow_functions_trailing_comma.json")).unwrap();
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

  fn visit_arrow_function_expression(
    &mut self,
    it: &oxc_ast::ast::ArrowFunctionExpression<'a>,
  ) {
    let params_span = it.params.span;
    let params_source_code = self.get_source_code(params_span);
    if params_source_code.ends_with(",)") {
      self.cache.push(CompatBox {
        start: params_span.start,
        end: params_span.end,
        code_seg: self.get_source_code(params_span).to_string(),
        compat: self.compat.clone(),
      });
    }
    oxc_ast::visit::walk::walk_arrow_function_expression(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_arrow_functions_trailing_comma() {
    let source_code = r##"
const materials = ['Hydrogen', 'Helium', 'Lithium', 'Beryllium'];
console.log(materials.map((material,) => material.length));    
"##;
    let allocator = Allocator::default();
    t_any(
      "arrow_functions_trailing_comma",
      source_code,
      &allocator,
      FunctionsVisitor::new,
    );
  }
}
