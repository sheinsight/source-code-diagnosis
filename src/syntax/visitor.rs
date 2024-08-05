use std::{marker::PhantomData, ops::Not};

use oxc_ast::{
  ast::{
    Argument, ArrayExpressionElement, BindingPatternKind, BindingProperty,
    Declaration, Expression, MethodDefinitionKind, ObjectPropertyKind,
  },
  AstKind, Visit,
};
use oxc_span::Span;
use oxc_syntax::{
  operator::{AssignmentOperator, BinaryOperator, LogicalOperator},
  scope::ScopeFlags,
  xml_entities,
};

use crate::syntax::compat::Compat;

use super::compat::CompatBox;

#[derive(Debug)]
pub struct SyntaxRecordVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
}

impl<'a> SyntaxRecordVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let operators_str = include_str!("./browser_compat_data/operators.json");
    let functions_str = include_str!("./browser_compat_data/functions.json");

    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }
}

impl<'a> Visit<'a> for SyntaxRecordVisitor<'a> {}
