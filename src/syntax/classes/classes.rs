use std::marker::PhantomData;

use oxc_ast::{
  ast::{ClassElement, MethodDefinitionKind, PropertyKey},
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
pub struct ClassesBrowserCompatMetadata {
  pub classes: Compat,
  pub constructor: Compat,
  pub extends: Compat,
  pub private_class_fields: Compat,
  pub private_class_fields_in: Compat,
  pub private_class_methods: Compat,
  pub public_class_fields: Compat,
  pub r#static: Compat,
  pub static_class_fields: Compat,
  pub static_initialization_blocks: Compat,
}

pub struct ClassesVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: ClassesBrowserCompatMetadata,
}

impl CommonTrait for ClassesVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> ClassesVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: ClassesBrowserCompatMetadata =
      from_str(include_str!("./classes.json")).unwrap();
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

impl<'a> Visit<'a> for ClassesVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_class(&mut self, it: &oxc_ast::ast::Class<'a>) {
    let code_seg = self.get_source_code(it.span).to_string();
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seg.clone(),
      compat: self.browser_compat_meta_data.classes.clone(),
    });
    if let Some(_) = it.super_class {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seg.clone(),
        compat: self.browser_compat_meta_data.extends.clone(),
      });
    }
    oxc_ast::visit::walk::walk_class(self, it);
  }

  fn visit_class_body(&mut self, it: &oxc_ast::ast::ClassBody<'a>) {
    for item in &it.body {
      if let ClassElement::MethodDefinition(ele) = item {
        if ele.kind == MethodDefinitionKind::Constructor {
          let code_seq = self.get_source_code(ele.span).to_string();
          self.cache.push(CompatBox {
            start: ele.span.start,
            end: ele.span.end,
            code_seg: code_seq,
            compat: self.browser_compat_meta_data.constructor.clone(),
          });
        }
      }
    }
    oxc_ast::visit::walk::walk_class_body(self, it);
  }

  fn visit_property_definition(
    &mut self,
    it: &oxc_ast::ast::PropertyDefinition<'a>,
  ) {
    let code_seq = self.get_source_code(it.span).to_string();
    if let PropertyKey::PrivateIdentifier(_) = it.key {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seq.clone(),
        compat: self.browser_compat_meta_data.private_class_fields.clone(),
      });
    }
    if it.r#static {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seq.clone(),
        compat: self.browser_compat_meta_data.r#static.clone(),
      });

      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seq.clone(),
        compat: self.browser_compat_meta_data.static_class_fields.clone(),
      });
    }
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seq.clone(),
      compat: self.browser_compat_meta_data.public_class_fields.clone(),
    });
    oxc_ast::visit::walk::walk_property_definition(self, it);
  }

  fn visit_method_definition(
    &mut self,
    it: &oxc_ast::ast::MethodDefinition<'a>,
  ) {
    let code_seq = self.get_source_code(it.span).to_string();
    if let PropertyKey::PrivateIdentifier(_) = it.key {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seq.clone(),
        compat: self.browser_compat_meta_data.private_class_methods.clone(),
      });
    }

    if it.r#static {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: code_seq.clone(),
        compat: self.browser_compat_meta_data.r#static.clone(),
      });
    }

    oxc_ast::visit::walk::walk_method_definition(self, it);
  }

  fn visit_private_in_expression(
    &mut self,
    it: &oxc_ast::ast::PrivateInExpression<'a>,
  ) {
    let code_seq = self.get_source_code(it.span).to_string();
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seq,
      compat: self
        .browser_compat_meta_data
        .private_class_fields_in
        .clone(),
    });
    oxc_ast::visit::walk::walk_private_in_expression(self, it);
  }

  fn visit_static_block(&mut self, it: &oxc_ast::ast::StaticBlock<'a>) {
    let code_seq = self.get_source_code(it.span).to_string();
    self.cache.push(CompatBox {
      start: it.span.start,
      end: it.span.end,
      code_seg: code_seq,
      compat: self
        .browser_compat_meta_data
        .static_initialization_blocks
        .clone(),
    });
    oxc_ast::visit::walk::walk_static_block(self, it);
  }
}

#[cfg(test)]
mod tests {
  use crate::syntax::operators_n::t::t_any;
  use oxc_allocator::Allocator;

  use super::*;

  #[test]
  fn should_exits_classes() {
    let source_code = r##"
class Rectangle {}    
"##;
    let allocator = Allocator::default();
    t_any("classes", source_code, &allocator, ClassesVisitor::new);
  }

  #[test]
  fn should_exits_classes_constructor() {
    let source_code = r##"
class Rectangle {
  constructor(){}
}    
"##;
    let allocator = Allocator::default();
    t_any("constructor", source_code, &allocator, ClassesVisitor::new);
  }

  #[test]
  fn should_exits_classes_extends() {
    let source_code = r##"
class Rectangle extends A {
}    
"##;
    let allocator = Allocator::default();
    t_any("extends", source_code, &allocator, ClassesVisitor::new);
  }

  #[test]
  fn should_exits_classes_private_class_fields() {
    let source_code = r##"
class Rectangle extends A {
  #height = 0;
}    
"##;
    let allocator = Allocator::default();
    t_any(
      "private_class_fields",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }

  #[test]
  fn should_exits_classes_private_class_methods() {
    let source_code = r##"
class Rectangle extends A {
  #privateMethod() {
  }
}    
"##;
    let allocator = Allocator::default();
    t_any(
      "private_class_methods",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }

  #[test]
  fn should_exits_classes_private_class_fields_in() {
    let source_code = r##"
class C {
  #x;
  constructor(x) {
    this.#x = x;
  }
  static getX(obj) {
    if (#x in obj) return obj.#x;

    return "obj must be an instance of C";
  }
}    
"##;
    let allocator = Allocator::default();
    t_any(
      "private_class_fields_in",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }

  #[test]
  fn should_exits_classes_public_class_fields() {
    let source_code = r##"
class C {
  age=12;
}
"##;
    let allocator = Allocator::default();
    t_any(
      "public_class_fields",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }

  #[test]
  fn should_exits_static() {
    let source_code = r##"
class ClassWithStaticMethod {
  staticProperty = 'someValue';
}
"##;
    let allocator = Allocator::default();
    t_any(
      "public_class_fields",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }

  #[test]
  fn should_exits_static_class_fields() {
    let source_code = r##"
class ClassWithStaticMethod {
  static Property = 'someValue';
}
"##;
    let allocator = Allocator::default();
    t_any(
      "static_class_fields",
      source_code,
      &allocator,
      ClassesVisitor::new,
    );
  }
}
