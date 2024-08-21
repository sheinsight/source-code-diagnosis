use oxc_ast::{
  ast::{
    ArrowFunctionExpression, Class, ClassBody, FormalParameter,
    FormalParameters, Function, IdentifierReference, MethodDefinition,
    PrivateInExpression, PropertyDefinition, StaticBlock,
    StaticMemberExpression,
  },
  visit::walk,
  Visit,
};
use oxc_semantic::ScopeFlags;

use super::common::Context;

#[derive(Debug)]
pub struct SyntaxVisitor<'a> {
  pub walk_class_body: Vec<fn(&mut Context, &ClassBody<'a>)>,
  pub walk_class: Vec<fn(&mut Context, &Class)>,
  pub walk_private_in_expression:
    Vec<fn(&mut Context, &PrivateInExpression<'a>)>,
  pub walk_property_definition: Vec<fn(&mut Context, &PropertyDefinition)>,
  pub walk_method_definition: Vec<fn(&mut Context, &MethodDefinition)>,
  pub walk_static_block: Vec<fn(&mut Context, &StaticBlock)>,
  pub walk_static_member_expression:
    Vec<fn(&mut Context, &StaticMemberExpression)>,
  pub walk_arrow_function_expression:
    Vec<fn(&mut Context, &ArrowFunctionExpression)>,
  pub walk_identifier_reference: Vec<fn(&mut Context, &IdentifierReference)>,
  pub walk_function: Vec<fn(&mut Context, &Function, &ScopeFlags, bool)>,
  pub walk_formal_parameter: Vec<fn(&mut Context, &FormalParameter)>,
  pub walk_formal_parameters: Vec<fn(&mut Context, &FormalParameters)>,
  pub context: Context<'a>,
  is_strict_mode: bool,
}

impl<'a> SyntaxVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let context = Context {
      source_code: source_code.to_string(),
      usage: Vec::new(),
      stack: Vec::new(),
    };
    Self {
      context: context,
      walk_class: Vec::new(),
      walk_function: Vec::new(),
      walk_class_body: Vec::new(),
      walk_static_block: Vec::new(),
      walk_formal_parameter: Vec::new(),
      walk_formal_parameters: Vec::new(),
      walk_method_definition: Vec::new(),
      walk_property_definition: Vec::new(),
      walk_identifier_reference: Vec::new(),
      walk_private_in_expression: Vec::new(),
      walk_static_member_expression: Vec::new(),
      walk_arrow_function_expression: Vec::new(),
      is_strict_mode: false,
    }
  }
}

impl<'a> Visit<'a> for SyntaxVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.context.stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.context.stack.pop();
  }

  fn visit_directive(&mut self, it: &oxc_ast::ast::Directive<'a>) {
    if it.directive == "use strict" {
      self.is_strict_mode = true;
    }
  }

  fn visit_method_definition(&mut self, it: &MethodDefinition<'a>) {
    for walk in &self.walk_method_definition {
      walk(&mut self.context, it);
    }
    walk::walk_method_definition(self, it);
  }

  fn visit_class_body(&mut self, it: &ClassBody<'a>) {
    for walk in &self.walk_class_body {
      walk(&mut self.context, it);
    }
    walk::walk_class_body(self, it);
  }

  fn visit_class(&mut self, it: &oxc_ast::ast::Class<'a>) {
    for walk in &self.walk_class {
      walk(&mut self.context, it);
    }
    walk::walk_class(self, it);
  }

  fn visit_private_in_expression(
    &mut self,
    it: &oxc_ast::ast::PrivateInExpression<'a>,
  ) {
    for walk in &self.walk_private_in_expression {
      walk(&mut self.context, it);
    }
    walk::walk_private_in_expression(self, it);
  }

  fn visit_property_definition(
    &mut self,
    it: &oxc_ast::ast::PropertyDefinition<'a>,
  ) {
    for walk in &self.walk_property_definition {
      walk(&mut self.context, it);
    }
    walk::walk_property_definition(self, it);
  }

  fn visit_static_block(&mut self, it: &oxc_ast::ast::StaticBlock<'a>) {
    for walk in &self.walk_static_block {
      walk(&mut self.context, it);
    }
    walk::walk_static_block(self, it);
  }

  fn visit_static_member_expression(
    &mut self,
    it: &oxc_ast::ast::StaticMemberExpression<'a>,
  ) {
    for walk in &self.walk_static_member_expression {
      walk(&mut self.context, it);
    }
    walk::walk_static_member_expression(self, it);
  }

  fn visit_arrow_function_expression(
    &mut self,
    it: &oxc_ast::ast::ArrowFunctionExpression<'a>,
  ) {
    for walk in &self.walk_arrow_function_expression {
      walk(&mut self.context, it);
    }
    walk::walk_arrow_function_expression(self, it);
  }

  fn visit_identifier_reference(
    &mut self,
    it: &oxc_ast::ast::IdentifierReference<'a>,
  ) {
    for walk in &self.walk_identifier_reference {
      walk(&mut self.context, it);
    }
    walk::walk_identifier_reference(self, it);
  }

  fn visit_function(
    &mut self,
    it: &oxc_ast::ast::Function<'a>,
    flags: oxc_semantic::ScopeFlags,
  ) {
    for walk in &self.walk_function {
      walk(&mut self.context, it, &flags, self.is_strict_mode);
    }
    walk::walk_function(self, it, flags);
  }

  fn visit_formal_parameter(&mut self, it: &oxc_ast::ast::FormalParameter<'a>) {
    for walk in &self.walk_formal_parameter {
      walk(&mut self.context, it);
    }
    walk::walk_formal_parameter(self, it);
  }

  fn visit_formal_parameters(&mut self, it: &FormalParameters<'a>) {
    for walk in &self.walk_formal_parameters {
      walk(&mut self.context, it);
    }
    walk::walk_formal_parameters(self, it);
  }
}

// #[cfg(test)]
// mod tests {

//   use crate::syntax::{classes::constructor, visitor::SyntaxVisitor};

//   crate::assert_ok_count! {
//     "classes_constructor",
//     |v: &mut SyntaxVisitor| v
//       .walk_class_body
//       .push(constructor::walk_class_body),

//     test_syntax_record_visitor,
//     r#"class A { constructor() { } }"#,
//     1,
//     test_syntax_record_visitor2,
//     r#" class A { constructor() { } }
//         class B { constructor() { } }"#,
//     2
//   }
// }
