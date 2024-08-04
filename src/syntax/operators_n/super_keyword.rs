use std::marker::PhantomData;

use oxc_ast::{
  ast::{CallExpression, Expression},
  AstKind, Visit,
};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct SuperKeywordBrowserCompatMetadata {
  pub super_keyword: Compat,
}

pub struct SuperKeywordVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: SuperKeywordBrowserCompatMetadata,
}

impl CommonTrait for SuperKeywordVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> SuperKeywordVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: SuperKeywordBrowserCompatMetadata =
      from_str(include_str!("./super_keyword.json")).unwrap();
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

impl<'a> Visit<'a> for SuperKeywordVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_call_expression(&mut self, it: &oxc_ast::ast::CallExpression<'a>) {
    let code_segment = self.get_source_code(it.span).to_string();
    if let Expression::Super(_) = it.callee {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_segment,
        compat: self.browser_compat_meta_data.super_keyword.clone(),
      });
    }
    oxc_ast::visit::walk::walk_call_expression(self, it);
  }

  fn visit_static_member_expression(
    &mut self,
    it: &oxc_ast::ast::StaticMemberExpression<'a>,
  ) {
    let code_segment = self.get_source_code(it.span).to_string();
    if let Expression::Super(_) = it.object {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_segment,
        compat: self.browser_compat_meta_data.super_keyword.clone(),
      });
    }
    oxc_ast::visit::walk::walk_static_member_expression(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_super_when_call_expression() {
    let source_code = r##"
class FooBar extends Foo {
  constructor(name, index) {
    super(name);
    this.index = index;
  }
}    
"##;
    let allocator = Allocator::default();
    t_any("super", source_code, &allocator, SuperKeywordVisitor::new);
  }

  #[test]
  fn should_exits_super_when_static_member_expression() {
    let source_code = r##"
class FooBar extends Foo {
  getFullName() {
    return this.name + super.getNameSeparator() + this.index;
  }
}
"##;
    let allocator = Allocator::default();
    t_any("super", source_code, &allocator, SuperKeywordVisitor::new);
  }
}
