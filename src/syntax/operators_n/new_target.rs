use std::marker::PhantomData;

use oxc_ast::{AstKind, Visit};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct NewTargetBrowserCompatMetadata {
  pub new_target: Compat,
}

pub struct NewTargetVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: NewTargetBrowserCompatMetadata,
}

impl CommonTrait for NewTargetVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> NewTargetVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: NewTargetBrowserCompatMetadata =
      from_str(include_str!("./new_target.json")).unwrap();
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

impl<'a> Visit<'a> for NewTargetVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_meta_property(&mut self, it: &oxc_ast::ast::MetaProperty<'a>) {
    if it.meta.name == "new" && it.property.name == "target" {
      let code_seg = self.get_source_code(it.span).to_string();
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg,
        compat: self.browser_compat_meta_data.new_target.clone(),
      });
    }
    oxc_ast::visit::walk::walk_meta_property(self, it);
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
function Foo() {
  if (!new.target) {
    throw new TypeError('calling Foo constructor without new is invalid');
  }
}
    "##;
    let allocator = Allocator::default();
    t_any("new_target", source_code, &allocator, NewTargetVisitor::new);
  }
}
