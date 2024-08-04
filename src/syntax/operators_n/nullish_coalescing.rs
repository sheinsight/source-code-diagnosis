use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use oxc_syntax::operator::LogicalOperator;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct NullishCoalescingBrowserCompatMetadata {
  pub nullish_coalescing: Compat,
}

pub struct NullishCoalescingVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: NullishCoalescingBrowserCompatMetadata,
}

impl CommonTrait for NullishCoalescingVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> NullishCoalescingVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: NullishCoalescingBrowserCompatMetadata =
      from_str(include_str!("./nullish_coalescing.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      browser_compat_meta_data: browser_compat_meta_data,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for NullishCoalescingVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_logical_expression(
    &mut self,
    it: &oxc_ast::ast::LogicalExpression<'a>,
  ) {
    let code_seg = self.get_source_code(it.span).to_string();
    if it.operator == LogicalOperator::Coalesce {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        compat: self.browser_compat_meta_data.nullish_coalescing.clone(),
        code_seg,
      });
    }
    oxc_ast::visit::walk::walk_logical_expression(self, it);
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
const foo = null ?? 'default string';
console.log(foo);


const baz = 0 ?? 42;
console.log(baz);


"##;
    let allocator = Allocator::default();
    t_any(
      "nullish_coalescing",
      source_code,
      &allocator,
      NullishCoalescingVisitor::new,
    );
  }
}
