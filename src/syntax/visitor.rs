use oxc_ast::{
  ast::{ClassBody, MethodDefinition, PrivateInExpression, PropertyDefinition},
  visit::walk,
  Visit,
};

use super::common::Context;

#[derive(Debug)]
pub struct SyntaxVisitor<'a> {
  pub walk_class_body: Vec<fn(&mut Context, &ClassBody<'a>)>,
  pub walk_class: Vec<fn(&mut Context, &oxc_ast::ast::Class)>,
  pub walk_private_in_expression:
    Vec<fn(&mut Context, &PrivateInExpression<'a>)>,
  pub walk_property_definition: Vec<fn(&mut Context, &PropertyDefinition)>,
  pub walk_method_definition: Vec<fn(&mut Context, &MethodDefinition)>,
  pub walk_static_block: Vec<fn(&mut Context, &oxc_ast::ast::StaticBlock)>,
  pub context: Context<'a>,
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
      walk_class_body: Vec::new(),
      walk_class: Vec::new(),
      walk_private_in_expression: Vec::new(),
      walk_property_definition: Vec::new(),
      walk_method_definition: Vec::new(),
      walk_static_block: Vec::new(),
    }
  }
}

impl<'a> Visit<'a> for SyntaxVisitor<'a> {
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
