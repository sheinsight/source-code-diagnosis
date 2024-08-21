use oxc_ast::{
  ast::{
    ArrayExpression, ArrowFunctionExpression, BooleanLiteral, CallExpression,
    Class, ClassBody, Directive, ExportNamedDeclaration, FormalParameter,
    FormalParameters, Function, IdentifierReference, ImportDeclaration,
    ImportExpression, MethodDefinition, NullLiteral, NumericLiteral,
    ObjectExpression, ObjectProperty, PrivateInExpression, Program,
    PropertyDefinition, RegExpLiteral, StaticBlock, StaticMemberExpression,
    StringLiteral, TemplateLiteral,
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
  pub walk_array_expression: Vec<fn(&mut Context, &ArrayExpression)>,
  pub walk_numeric_literal: Vec<fn(&mut Context, &NumericLiteral)>,
  pub walk_boolean_literal: Vec<fn(&mut Context, &BooleanLiteral)>,
  pub walk_program: Vec<fn(&mut Context, &Program)>,
  pub walk_directive: Vec<fn(&mut Context, &Directive)>,
  pub walk_null_literal: Vec<fn(&mut Context, &NullLiteral)>,
  pub walk_reg_exp_literal: Vec<fn(&mut Context, &RegExpLiteral)>,
  pub walk_object_property: Vec<fn(&mut Context, &ObjectProperty)>,
  pub walk_string_literal: Vec<fn(&mut Context, &StringLiteral)>,
  pub walk_call_expression: Vec<fn(&mut Context, &CallExpression)>,
  pub walk_template_literal: Vec<fn(&mut Context, &TemplateLiteral)>,
  pub walk_import_expression: Vec<fn(&mut Context, &ImportExpression)>,
  pub walk_object_expression: Vec<fn(&mut Context, &ObjectExpression)>,
  pub walk_import_declaration: Vec<fn(&mut Context, &ImportDeclaration)>,
  pub walk_export_named_declaration:
    Vec<fn(&mut Context, &ExportNamedDeclaration)>,
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
      walk_program: Vec::new(),
      walk_function: Vec::new(),
      walk_directive: Vec::new(),
      walk_class_body: Vec::new(),
      walk_static_block: Vec::new(),
      walk_null_literal: Vec::new(),
      walk_string_literal: Vec::new(),
      walk_call_expression: Vec::new(),
      walk_object_property: Vec::new(),
      walk_reg_exp_literal: Vec::new(),
      walk_numeric_literal: Vec::new(),
      walk_boolean_literal: Vec::new(),
      walk_formal_parameter: Vec::new(),
      walk_template_literal: Vec::new(),
      walk_array_expression: Vec::new(),
      walk_formal_parameters: Vec::new(),
      walk_method_definition: Vec::new(),
      walk_object_expression: Vec::new(),
      walk_import_expression: Vec::new(),
      walk_import_declaration: Vec::new(),
      walk_property_definition: Vec::new(),
      walk_identifier_reference: Vec::new(),
      walk_private_in_expression: Vec::new(),
      walk_export_named_declaration: Vec::new(),
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

  fn visit_reg_exp_literal(&mut self, it: &RegExpLiteral<'a>) {
    for walk in &self.walk_reg_exp_literal {
      walk(&mut self.context, it);
    }
    walk::walk_reg_exp_literal(self, it);
  }

  fn visit_program(&mut self, it: &Program<'a>) {
    for walk in &self.walk_program {
      walk(&mut self.context, it);
    }
    walk::walk_program(self, it);
  }

  fn visit_directive(&mut self, it: &oxc_ast::ast::Directive<'a>) {
    if it.directive == "use strict" {
      self.is_strict_mode = true;
    }
    for walk in &self.walk_directive {
      walk(&mut self.context, it);
    }
    walk::walk_directive(self, it);
  }

  fn visit_template_literal(&mut self, it: &oxc_ast::ast::TemplateLiteral<'a>) {
    for walk in &self.walk_template_literal {
      walk(&mut self.context, it);
    }
    walk::walk_template_literal(self, it);
  }

  fn visit_import_expression(&mut self, it: &ImportExpression<'a>) {
    for walk in &self.walk_import_expression {
      walk(&mut self.context, it);
    }
    walk::walk_import_expression(self, it);
  }

  fn visit_string_literal(&mut self, it: &StringLiteral<'a>) {
    for walk in &self.walk_string_literal {
      walk(&mut self.context, it);
    }
    walk::walk_string_literal(self, it);
  }

  fn visit_call_expression(&mut self, it: &CallExpression<'a>) {
    for walk in &self.walk_call_expression {
      walk(&mut self.context, it);
    }
    walk::walk_call_expression(self, it);
  }

  fn visit_import_declaration(&mut self, it: &ImportDeclaration<'a>) {
    for walk in &self.walk_import_declaration {
      walk(&mut self.context, it);
    }
    walk::walk_import_declaration(self, it);
  }

  fn visit_export_named_declaration(
    &mut self,
    it: &ExportNamedDeclaration<'a>,
  ) {
    for walk in &self.walk_export_named_declaration {
      walk(&mut self.context, it);
    }
    walk::walk_export_named_declaration(self, it);
  }

  fn visit_object_expression(&mut self, it: &ObjectExpression<'a>) {
    for walk in &self.walk_object_expression {
      walk(&mut self.context, it);
    }
    walk::walk_object_expression(self, it);
  }

  fn visit_object_property(&mut self, it: &ObjectProperty<'a>) {
    for walk in &self.walk_object_property {
      walk(&mut self.context, it);
    }
    walk::walk_object_property(self, it);
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

  fn visit_array_expression(&mut self, it: &ArrayExpression<'a>) {
    for walk in &self.walk_array_expression {
      walk(&mut self.context, it);
    }
    walk::walk_array_expression(self, it);
  }

  fn visit_numeric_literal(&mut self, it: &NumericLiteral<'a>) {
    for walk in &self.walk_numeric_literal {
      walk(&mut self.context, it);
    }
    walk::walk_numeric_literal(self, it);
  }

  fn visit_boolean_literal(&mut self, it: &BooleanLiteral) {
    for walk in &self.walk_boolean_literal {
      walk(&mut self.context, it);
    }
    walk::walk_boolean_literal(self, it);
  }

  fn visit_null_literal(&mut self, it: &NullLiteral) {
    for walk in &self.walk_null_literal {
      walk(&mut self.context, it);
    }
    walk::walk_null_literal(self, it);
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
