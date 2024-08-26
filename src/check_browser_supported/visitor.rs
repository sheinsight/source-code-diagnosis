use oxc_ast::{
  ast::{
    ArrayAssignmentTarget, ArrayExpression, ArrayPattern,
    ArrowFunctionExpression, AssignmentExpression, AwaitExpression,
    BinaryExpression, BlockStatement, BooleanLiteral, BreakStatement,
    CallExpression, CatchClause, ChainExpression, Class, ClassBody,
    ComputedMemberExpression, ConditionalExpression, ContinueStatement,
    DebuggerStatement, Directive, DoWhileStatement, EmptyStatement,
    ExportAllDeclaration, ExportDefaultDeclaration, ExportNamedDeclaration,
    ForInStatement, ForOfStatement, ForStatement, FormalParameter,
    FormalParameters, Function, IdentifierReference, IfStatement,
    ImportAttribute, ImportDeclaration, ImportExpression, LabeledStatement,
    LogicalExpression, MetaProperty, MethodDefinition, NewExpression,
    NullLiteral, NumericLiteral, ObjectExpression, ObjectPattern,
    ObjectProperty, ParenthesizedExpression, PrivateInExpression, Program,
    PropertyDefinition, RegExpLiteral, ReturnStatement, SequenceExpression,
    SpreadElement, StaticBlock, StaticMemberExpression, StringLiteral,
    SwitchStatement, TemplateLiteral, ThisExpression, ThrowStatement,
    TryStatement, UnaryExpression, UpdateExpression, VariableDeclaration,
    VariableDeclarator, WhileStatement, WithClause, WithStatement,
    YieldExpression,
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
  pub walk_function: Vec<fn(&mut Context, &Function, &ScopeFlags)>,
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
  pub walk_assignment_expression: Vec<fn(&mut Context, &AssignmentExpression)>,
  pub walk_binary_expression: Vec<fn(&mut Context, &BinaryExpression)>,
  pub walk_variable_declarator: Vec<fn(&mut Context, &VariableDeclarator)>,
  pub walk_await_expression: Vec<fn(&mut Context, &AwaitExpression)>,
  pub walk_unary_expression: Vec<fn(&mut Context, &UnaryExpression)>,
  pub walk_sequence_expression: Vec<fn(&mut Context, &SequenceExpression)>,
  pub walk_update_expression: Vec<fn(&mut Context, &UpdateExpression)>,
  pub walk_conditional_expression:
    Vec<fn(&mut Context, &ConditionalExpression)>,
  pub walk_computed_member_expression:
    Vec<fn(&mut Context, &ComputedMemberExpression)>,
  pub walk_object_pattern: Vec<fn(&mut Context, &ObjectPattern)>,
  pub walk_array_pattern: Vec<fn(&mut Context, &ArrayPattern)>,
  pub walk_array_assignment_target:
    Vec<fn(&mut Context, &ArrayAssignmentTarget)>,
  pub walk_parenthesized_expression:
    Vec<fn(&mut Context, &ParenthesizedExpression)>,
  pub walk_meta_property: Vec<fn(&mut Context, &MetaProperty)>,
  pub walk_logical_expression: Vec<fn(&mut Context, &LogicalExpression)>,
  pub walk_new_expression: Vec<fn(&mut Context, &NewExpression)>,
  pub walk_chain_expression: Vec<fn(&mut Context, &ChainExpression)>,
  pub walk_spread_element: Vec<fn(&mut Context, &SpreadElement)>,
  pub walk_this_expression: Vec<fn(&mut Context, &ThisExpression)>,
  pub walk_yield_expression: Vec<fn(&mut Context, &YieldExpression)>,
  pub walk_block_statement: Vec<fn(&mut Context, &BlockStatement)>,
  pub walk_break_statement: Vec<fn(&mut Context, &BreakStatement)>,
  pub walk_empty_statement: Vec<fn(&mut Context, &EmptyStatement)>,
  pub walk_do_while_statement: Vec<fn(&mut Context, &DoWhileStatement)>,
  pub walk_debugger_statement: Vec<fn(&mut Context, &DebuggerStatement)>,
  pub walk_continue_statement: Vec<fn(&mut Context, &ContinueStatement)>,
  pub walk_variable_declaration: Vec<fn(&mut Context, &VariableDeclaration)>,
  pub walk_export_default_declaration:
    Vec<fn(&mut Context, &ExportDefaultDeclaration)>,
  pub walk_export_all_declaration: Vec<fn(&mut Context, &ExportAllDeclaration)>,
  pub walk_for_of_statement: Vec<fn(&mut Context, &ForOfStatement)>,
  pub walk_for_in_statement: Vec<fn(&mut Context, &ForInStatement)>,
  pub walk_for_statement: Vec<fn(&mut Context, &ForStatement)>,
  pub walk_if_statement: Vec<fn(&mut Context, &IfStatement)>,
  pub walk_import_attribute: Vec<fn(&mut Context, &ImportAttribute)>,
  pub walk_with_clause: Vec<fn(&mut Context, it: &WithClause)>,
  pub walk_labeled_statement: Vec<fn(&mut Context, it: &LabeledStatement)>,
  pub walk_return_statement: Vec<fn(&mut Context, it: &ReturnStatement)>,
  pub walk_switch_statement: Vec<fn(&mut Context, it: &SwitchStatement)>,
  pub walk_throw_statement: Vec<fn(&mut Context, it: &ThrowStatement)>,
  pub walk_catch_clause: Vec<fn(&mut Context, it: &CatchClause)>,
  pub walk_try_statement: Vec<fn(&mut Context, it: &TryStatement)>,
  pub walk_while_statement: Vec<fn(&mut Context, it: &WhileStatement)>,
  pub walk_with_statement: Vec<fn(&mut Context, it: &WithStatement)>,
  pub context: Context<'a>,
}

impl<'a> SyntaxVisitor<'a> {
  pub fn new(source_code: &'a str, file_path: &'a str) -> Self {
    let context = Context::new(source_code, file_path);
    Self {
      context: context,
      walk_class: Vec::new(),
      walk_program: Vec::new(),
      walk_function: Vec::new(),
      walk_directive: Vec::new(),
      walk_class_body: Vec::new(),
      walk_with_clause: Vec::new(),
      walk_static_block: Vec::new(),
      walk_if_statement: Vec::new(),
      walk_null_literal: Vec::new(),
      walk_catch_clause: Vec::new(),
      walk_meta_property: Vec::new(),
      walk_array_pattern: Vec::new(),
      walk_for_statement: Vec::new(),
      walk_with_statement: Vec::new(),
      walk_try_statement: Vec::new(),
      walk_string_literal: Vec::new(),
      walk_spread_element: Vec::new(),
      walk_object_pattern: Vec::new(),
      walk_new_expression: Vec::new(),
      walk_block_statement: Vec::new(),
      walk_call_expression: Vec::new(),
      walk_object_property: Vec::new(),
      walk_this_expression: Vec::new(),
      walk_reg_exp_literal: Vec::new(),
      walk_numeric_literal: Vec::new(),
      walk_boolean_literal: Vec::new(),
      walk_break_statement: Vec::new(),
      walk_while_statement: Vec::new(),
      walk_empty_statement: Vec::new(),
      walk_throw_statement: Vec::new(),
      walk_switch_statement: Vec::new(),
      walk_return_statement: Vec::new(),
      walk_for_of_statement: Vec::new(),
      walk_formal_parameter: Vec::new(),
      walk_chain_expression: Vec::new(),
      walk_yield_expression: Vec::new(),
      walk_import_attribute: Vec::new(),
      walk_unary_expression: Vec::new(),
      walk_template_literal: Vec::new(),
      walk_array_expression: Vec::new(),
      walk_await_expression: Vec::new(),
      walk_for_in_statement: Vec::new(),
      walk_binary_expression: Vec::new(),
      walk_formal_parameters: Vec::new(),
      walk_method_definition: Vec::new(),
      walk_labeled_statement: Vec::new(),
      walk_object_expression: Vec::new(),
      walk_import_expression: Vec::new(),
      walk_update_expression: Vec::new(),
      walk_import_declaration: Vec::new(),
      walk_debugger_statement: Vec::new(),
      walk_continue_statement: Vec::new(),
      walk_logical_expression: Vec::new(),
      walk_do_while_statement: Vec::new(),
      walk_variable_declarator: Vec::new(),
      walk_sequence_expression: Vec::new(),
      walk_property_definition: Vec::new(),
      walk_identifier_reference: Vec::new(),
      walk_variable_declaration: Vec::new(),
      walk_private_in_expression: Vec::new(),
      walk_assignment_expression: Vec::new(),
      walk_export_all_declaration: Vec::new(),
      walk_conditional_expression: Vec::new(),
      walk_array_assignment_target: Vec::new(),
      walk_parenthesized_expression: Vec::new(),
      walk_export_named_declaration: Vec::new(),
      walk_static_member_expression: Vec::new(),
      walk_arrow_function_expression: Vec::new(),
      walk_computed_member_expression: Vec::new(),
      walk_export_default_declaration: Vec::new(),
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

  fn visit_for_of_statement(&mut self, it: &ForOfStatement<'a>) {
    for walk in &self.walk_for_of_statement {
      walk(&mut self.context, it);
    }
    walk::walk_for_of_statement(self, it);
  }

  fn visit_for_statement(&mut self, it: &ForStatement<'a>) {
    for walk in &self.walk_for_statement {
      walk(&mut self.context, it);
    }
    walk::walk_for_statement(self, it);
  }

  fn visit_program(&mut self, it: &Program<'a>) {
    for walk in &self.walk_program {
      walk(&mut self.context, it);
    }
    walk::walk_program(self, it);
  }

  fn visit_while_statement(&mut self, it: &WhileStatement<'a>) {
    for walk in &self.walk_while_statement {
      walk(&mut self.context, it);
    }
    walk::walk_while_statement(self, it);
  }

  fn visit_with_statement(&mut self, it: &WithStatement<'a>) {
    for walk in &self.walk_with_statement {
      walk(&mut self.context, it);
    }
    walk::walk_with_statement(self, it);
  }

  fn visit_try_statement(&mut self, it: &TryStatement<'a>) {
    for walk in &self.walk_try_statement {
      walk(&mut self.context, it);
    }
    walk::walk_try_statement(self, it);
  }

  fn visit_labeled_statement(&mut self, it: &LabeledStatement<'a>) {
    for walk in &self.walk_labeled_statement {
      walk(&mut self.context, it);
    }
    walk::walk_labeled_statement(self, it);
  }

  fn visit_return_statement(&mut self, it: &ReturnStatement<'a>) {
    for walk in &self.walk_return_statement {
      walk(&mut self.context, it);
    }
    walk::walk_return_statement(self, it);
  }

  fn visit_this_expression(&mut self, it: &ThisExpression) {
    for walk in &self.walk_this_expression {
      walk(&mut self.context, it);
    }
    walk::walk_this_expression(self, it);
  }

  fn visit_for_in_statement(&mut self, it: &oxc_ast::ast::ForInStatement<'a>) {
    for walk in &self.walk_for_in_statement {
      walk(&mut self.context, it);
    }
    walk::walk_for_in_statement(self, it);
  }

  fn visit_switch_statement(&mut self, it: &SwitchStatement<'a>) {
    for walk in &self.walk_switch_statement {
      walk(&mut self.context, it);
    }
    walk::walk_switch_statement(self, it);
  }

  fn visit_throw_statement(&mut self, it: &ThrowStatement<'a>) {
    for walk in &self.walk_throw_statement {
      walk(&mut self.context, it);
    }
    walk::walk_throw_statement(self, it);
  }

  fn visit_catch_clause(&mut self, it: &CatchClause<'a>) {
    for walk in &self.walk_catch_clause {
      walk(&mut self.context, it);
    }
    walk::walk_catch_clause(self, it);
  }

  fn visit_with_clause(&mut self, it: &WithClause<'a>) {
    for walk in &self.walk_with_clause {
      walk(&mut self.context, it);
    }
    walk::walk_with_clause(self, it);
  }

  fn visit_array_pattern(&mut self, it: &ArrayPattern<'a>) {
    for walk in &self.walk_array_pattern {
      walk(&mut self.context, it);
    }
    walk::walk_array_pattern(self, it);
  }

  fn visit_yield_expression(&mut self, it: &YieldExpression<'a>) {
    for walk in &self.walk_yield_expression {
      walk(&mut self.context, it);
    }
    walk::walk_yield_expression(self, it);
  }

  fn visit_do_while_statement(&mut self, it: &DoWhileStatement<'a>) {
    for walk in &self.walk_do_while_statement {
      walk(&mut self.context, it);
    }
    walk::walk_do_while_statement(self, it);
  }

  fn visit_export_default_declaration(
    &mut self,
    it: &ExportDefaultDeclaration<'a>,
  ) {
    for walk in &self.walk_export_default_declaration {
      walk(&mut self.context, it);
    }
    walk::walk_export_default_declaration(self, it);
  }

  fn visit_if_statement(&mut self, it: &IfStatement<'a>) {
    for walk in &self.walk_if_statement {
      walk(&mut self.context, it);
    }
    walk::walk_if_statement(self, it);
  }

  fn visit_import_attribute(&mut self, it: &ImportAttribute<'a>) {
    for walk in &self.walk_import_attribute {
      walk(&mut self.context, it);
    }
    walk::walk_import_attribute(self, it);
  }

  fn visit_export_all_declaration(
    &mut self,
    it: &oxc_ast::ast::ExportAllDeclaration<'a>,
  ) {
    for walk in &self.walk_export_all_declaration {
      walk(&mut self.context, it);
    }
    walk::walk_export_all_declaration(self, it);
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

  fn visit_empty_statement(&mut self, it: &EmptyStatement) {
    for walk in &self.walk_empty_statement {
      walk(&mut self.context, it);
    }
    walk::walk_empty_statement(self, it);
  }

  fn visit_break_statement(&mut self, it: &BreakStatement<'a>) {
    for walk in &self.walk_break_statement {
      walk(&mut self.context, it);
    }
    walk::walk_break_statement(self, it);
  }

  fn visit_sequence_expression(&mut self, it: &SequenceExpression<'a>) {
    for walk in &self.walk_sequence_expression {
      walk(&mut self.context, it);
    }
    walk::walk_sequence_expression(self, it);
  }

  fn visit_continue_statement(&mut self, it: &ContinueStatement<'a>) {
    for walk in &self.walk_continue_statement {
      walk(&mut self.context, it);
    }
    walk::walk_continue_statement(self, it);
  }

  fn visit_block_statement(&mut self, it: &BlockStatement<'a>) {
    for walk in &self.walk_block_statement {
      walk(&mut self.context, it);
    }
    walk::walk_block_statement(self, it);
  }

  fn visit_debugger_statement(&mut self, it: &DebuggerStatement) {
    for walk in &self.walk_debugger_statement {
      walk(&mut self.context, it);
    }
    walk::walk_debugger_statement(self, it);
  }

  fn visit_computed_member_expression(
    &mut self,
    it: &ComputedMemberExpression<'a>,
  ) {
    for walk in &self.walk_computed_member_expression {
      walk(&mut self.context, it);
    }
    walk::walk_computed_member_expression(self, it);
  }

  fn visit_chain_expression(&mut self, it: &ChainExpression<'a>) {
    for walk in &self.walk_chain_expression {
      walk(&mut self.context, it);
    }
    walk::walk_chain_expression(self, it);
  }

  fn visit_new_expression(&mut self, it: &NewExpression<'a>) {
    for walk in &self.walk_new_expression {
      walk(&mut self.context, it);
    }
    walk::walk_new_expression(self, it);
  }

  fn visit_meta_property(&mut self, it: &MetaProperty<'a>) {
    for walk in &self.walk_meta_property {
      walk(&mut self.context, it);
    }
    walk::walk_meta_property(self, it);
  }

  fn visit_logical_expression(&mut self, it: &LogicalExpression<'a>) {
    for walk in &self.walk_logical_expression {
      walk(&mut self.context, it);
    }
    walk::walk_logical_expression(self, it);
  }

  fn visit_spread_element(&mut self, it: &SpreadElement<'a>) {
    for walk in &self.walk_spread_element {
      walk(&mut self.context, it);
    }
    walk::walk_spread_element(self, it);
  }

  fn visit_object_pattern(&mut self, it: &ObjectPattern<'a>) {
    for walk in &self.walk_object_pattern {
      walk(&mut self.context, it);
    }
    walk::walk_object_pattern(self, it);
  }

  fn visit_array_assignment_target(&mut self, it: &ArrayAssignmentTarget<'a>) {
    for walk in &self.walk_array_assignment_target {
      walk(&mut self.context, it);
    }
    walk::walk_array_assignment_target(self, it);
  }

  fn visit_variable_declaration(&mut self, it: &VariableDeclaration<'a>) {
    for walk in &self.walk_variable_declaration {
      walk(&mut self.context, it);
    }
    walk::walk_variable_declaration(self, it);
  }

  fn visit_directive(&mut self, it: &oxc_ast::ast::Directive<'a>) {
    if it.directive == "use strict" {
      self.context.is_strict_mode = true;
    }
    for walk in &self.walk_directive {
      walk(&mut self.context, it);
    }
    walk::walk_directive(self, it);
  }

  fn visit_parenthesized_expression(
    &mut self,
    it: &ParenthesizedExpression<'a>,
  ) {
    for walk in &self.walk_parenthesized_expression {
      walk(&mut self.context, it);
    }
    walk::walk_parenthesized_expression(self, it);
  }

  fn visit_unary_expression(&mut self, it: &oxc_ast::ast::UnaryExpression<'a>) {
    for walk in &self.walk_unary_expression {
      walk(&mut self.context, it);
    }
    walk::walk_unary_expression(self, it);
  }

  fn visit_update_expression(
    &mut self,
    it: &oxc_ast::ast::UpdateExpression<'a>,
  ) {
    for walk in &self.walk_update_expression {
      walk(&mut self.context, it);
    }
    walk::walk_update_expression(self, it);
  }

  fn visit_template_literal(&mut self, it: &oxc_ast::ast::TemplateLiteral<'a>) {
    for walk in &self.walk_template_literal {
      walk(&mut self.context, it);
    }
    walk::walk_template_literal(self, it);
  }

  fn visit_conditional_expression(&mut self, it: &ConditionalExpression<'a>) {
    for walk in &self.walk_conditional_expression {
      walk(&mut self.context, it);
    }
    walk::walk_conditional_expression(self, it);
  }

  fn visit_variable_declarator(&mut self, it: &VariableDeclarator<'a>) {
    for walk in &self.walk_variable_declarator {
      walk(&mut self.context, it);
    }
    walk::walk_variable_declarator(self, it);
  }

  fn visit_import_expression(&mut self, it: &ImportExpression<'a>) {
    for walk in &self.walk_import_expression {
      walk(&mut self.context, it);
    }
    walk::walk_import_expression(self, it);
  }

  fn visit_assignment_expression(&mut self, it: &AssignmentExpression<'a>) {
    for walk in &self.walk_assignment_expression {
      walk(&mut self.context, it);
    }
    walk::walk_assignment_expression(self, it);
  }

  fn visit_binary_expression(&mut self, it: &BinaryExpression<'a>) {
    for walk in &self.walk_binary_expression {
      walk(&mut self.context, it);
    }
    walk::walk_binary_expression(self, it);
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

  fn visit_await_expression(&mut self, it: &AwaitExpression<'a>) {
    for walk in &self.walk_await_expression {
      walk(&mut self.context, it);
    }
    walk::walk_await_expression(self, it);
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
      walk(&mut self.context, it, &flags);
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
