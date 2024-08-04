use std::marker::PhantomData;

use oxc_ast::{
  ast::{BindingPattern, BindingPatternKind, Expression},
  AstKind, Visit,
};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::{
  compat::{Compat, CompatBox},
  operators_n::common_trait::CommonTrait,
};

#[derive(Debug, Deserialize)]
pub struct FunctionsBrowserCompatMetadata {
  pub functions: Compat,
  pub arguments: Compat,
  pub arguments_callee: Compat,
  pub arguments_length: Compat,
  pub arguments_iterator: Compat,
  pub arrow_functions: Compat,
  pub arrow_functions_trailing_comma: Compat,
  pub block_level_functions: Compat,
  pub default_parameters: Compat,
  pub default_parameters_destructured_parameter_with_default_value_assignment:
    Compat,
  pub default_parameters_parameters_without_defaults_after_default_parameters:
    Compat,
  pub getter: Compat,
  pub getter_computed_property_names: Compat,
  pub method_definitions: Compat,
  pub async_generator_methods: Compat,
  pub async_methods: Compat,
  pub generator_methods_not_constructable: Compat,
  pub rest_parameters: Compat,
  pub rest_parameters_destructuring: Compat,
  pub setter: Compat,
  pub setter_computed_property_names: Compat,
}

pub struct FunctionsVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: FunctionsBrowserCompatMetadata,
}

impl CommonTrait for FunctionsVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> FunctionsVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: FunctionsBrowserCompatMetadata =
      from_str(include_str!("./functions.json")).unwrap();
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
    let code_seg = self.get_source_code(it.span).to_string();
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seg.clone(),
      compat: self.browser_compat_meta_data.functions.clone(),
    });

    oxc_ast::visit::walk::walk_function(self, it, flags);
  }

  fn visit_computed_member_expression(
    &mut self,
    it: &oxc_ast::ast::ComputedMemberExpression<'a>,
  ) {
    let code_seg = self.get_source_code(it.span).to_string();
    if let Expression::ComputedMemberExpression(_) = it.object {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg.clone(),
        compat: self.browser_compat_meta_data.arguments.clone(),
      });
    }

    oxc_ast::visit::walk::walk_computed_member_expression(self, it);
  }

  fn visit_identifier_name(&mut self, it: &oxc_ast::ast::IdentifierName<'a>) {
    if it.name == "arguments" {
      let code_seg = self.get_source_code(it.span).to_string();
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg.clone(),
        compat: self.browser_compat_meta_data.arguments.clone(),
      });
    }
    oxc_ast::visit::walk::walk_identifier_name(self, it);
  }

  fn visit_static_member_expression(
    &mut self,
    it: &oxc_ast::ast::StaticMemberExpression<'a>,
  ) {
    let is_arguments = if let Expression::Identifier(obj) = &it.object {
      if obj.name == "arguments" {
        true
      } else {
        false
      }
    } else {
      false
    };

    let is_callee = if it.property.name == "callee" {
      true
    } else {
      false
    };

    let is_length = if it.property.name == "length" {
      true
    } else {
      false
    };

    if is_arguments {
      let code_seg = self.get_source_code(it.span).to_string();
      if is_callee {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seg.clone(),
          compat: self.browser_compat_meta_data.arguments_callee.clone(),
        });
      }
      if is_length {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seg.clone(),
          compat: self.browser_compat_meta_data.arguments_length.clone(),
        });
      }
    }
    oxc_ast::visit::walk::walk_static_member_expression(self, it);
  }

  fn visit_arrow_function_expression(
    &mut self,
    it: &oxc_ast::ast::ArrowFunctionExpression<'a>,
  ) {
    let code_seg = self.get_source_code(it.span).to_string();
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seg.clone(),
      compat: self.browser_compat_meta_data.arrow_functions.clone(),
    });

    let params_span = it.params.span;
    let params_source_code = self.get_source_code(params_span);
    if params_source_code.ends_with(",)") {
      self.cache.push(CompatBox {
        start: params_span.start,
        end: params_span.end,
        code_seg: self.get_source_code(params_span).to_string(),
        compat: self
          .browser_compat_meta_data
          .arrow_functions_trailing_comma
          .clone(),
      });
    }
    oxc_ast::visit::walk::walk_arrow_function_expression(self, it);
  }

  fn visit_formal_parameter(&mut self, it: &oxc_ast::ast::FormalParameter<'a>) {
    let code_seg = self.get_source_code(it.span).to_string();

    match it.pattern.kind {
      BindingPatternKind::AssignmentPattern(_) => {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seg.clone(),
          compat: self
          .browser_compat_meta_data
          .default_parameters_destructured_parameter_with_default_value_assignment
          .clone(),
        });
      }
      BindingPatternKind::ObjectPattern(_)
      | BindingPatternKind::ArrayPattern(_) => {
        self.cache.push(CompatBox {
          start: it.span.start,
          end: it.span.end,
          code_seg: code_seg.clone(),
          compat: self
          .browser_compat_meta_data
          .default_parameters_parameters_without_defaults_after_default_parameters
          .clone(),
        });
      }
      BindingPatternKind::BindingIdentifier(_) => {}
    }

    oxc_ast::visit::walk::walk_formal_parameter(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_test() {
    let source_code = r##""##;
    let allocator = Allocator::default();
    t_any("tmp", source_code, &allocator, FunctionsVisitor::new);
  }
}
