use std::marker::PhantomData;

use oxc_ast::{
  ast::{Argument, ArrayExpressionElement, ObjectPropertyKind},
  AstKind, Visit,
};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct SpreadBrowserCompatMetadata {
  pub spread: Compat,
  pub spread_in_arrays: Compat,
  pub spread_in_function_calls: Compat,
  pub spread_in_object_literals: Compat,
}

pub struct SpreadVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: SpreadBrowserCompatMetadata,
}

impl CommonTrait for SpreadVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> SpreadVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: SpreadBrowserCompatMetadata =
      from_str(include_str!("./spread.json")).unwrap();
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

impl<'a> Visit<'a> for SpreadVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_array_expression_element(
    &mut self,
    it: &oxc_ast::ast::ArrayExpressionElement<'a>,
  ) {
    if let ArrayExpressionElement::SpreadElement(arg) = it {
      self.cache.push(CompatBox {
        start: arg.span.start,
        end: arg.span.end,
        code_seg: self.get_source_code(arg.span).to_string(),
        compat: self.browser_compat_meta_data.spread_in_arrays.clone(),
      });
    }
    oxc_ast::visit::walk::walk_array_expression_element(self, it);
  }

  fn visit_object_expression(
    &mut self,
    expr: &oxc_ast::ast::ObjectExpression<'a>,
  ) {
    for prop in expr.properties.iter() {
      if let ObjectPropertyKind::SpreadProperty(p) = prop {
        self.cache.push(CompatBox {
          start: p.span.start,
          end: p.span.end,
          code_seg: self.get_source_code(p.span).to_string(),
          compat: self
            .browser_compat_meta_data
            .spread_in_object_literals
            .clone(),
        });
      }
    }

    oxc_ast::visit::walk::walk_object_expression(self, expr);
  }

  fn visit_call_expression(&mut self, expr: &oxc_ast::ast::CallExpression<'a>) {
    for arg in &expr.arguments {
      if let Argument::SpreadElement(arg) = arg {
        self.cache.push(CompatBox {
          start: arg.span.start,
          end: arg.span.end,
          code_seg: self.get_source_code(arg.span).to_string(),
          compat: self
            .browser_compat_meta_data
            .spread_in_function_calls
            .clone(),
        });
      }
    }
    oxc_ast::visit::walk::walk_call_expression(self, expr);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_spread_in_arrays() {
    let source_code = r##"
const parts = ["shoulders", "knees"];
const lyrics = ["head", ...parts, "and", "toes"];    
"##;
    let allocator = Allocator::default();
    t_any(
      "spread_in_arrays",
      source_code,
      &allocator,
      SpreadVisitor::new,
    );
  }

  #[test]
  fn should_exits_spread_in_function_calls() {
    let source_code = r##"
function myFunction(x, y, z) {}
const args = [0, 1, 2];
myFunction(...args); 
    "##;
    let allocator = Allocator::default();
    t_any(
      "spread_in_function_calls",
      source_code,
      &allocator,
      SpreadVisitor::new,
    );
  }

  #[test]
  fn should_exits_spread_in_object_literals() {
    let source_code = r##"
const obj1 = { foo: "bar", x: 42 };
const obj2 = { bar: "baz", y: 13 };

const mergedObj = { ...obj1, ...obj2 };
        "##;
    let allocator = Allocator::default();
    t_any(
      "spread_in_object_literals",
      source_code,
      &allocator,
      SpreadVisitor::new,
    );
  }
}
