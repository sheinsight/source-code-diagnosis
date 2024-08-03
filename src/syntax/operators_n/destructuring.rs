use std::marker::PhantomData;

use oxc_ast::{visit::walk, AstKind, Visit};
use oxc_span::Span;
use serde::Deserialize;
use serde_json::from_str;

use crate::syntax::compat::{Compat, CompatBox};

use super::common_trait::CommonTrait;

#[derive(Debug, Deserialize)]
pub struct DestructuringBrowserCompatMetadata {
  destructuring: Compat,
  rest_in_arrays: Compat,
  rest_in_objects: Compat,
  computed_property_names: Compat,
}

pub struct DestructuringVisitor<'a> {
  pub cache: Vec<CompatBox>,
  parent_stack: Vec<AstKind<'a>>,
  source_code: &'a str,
  _phantom: PhantomData<&'a ()>,
  browser_compat_meta_data: DestructuringBrowserCompatMetadata,
}

impl CommonTrait for DestructuringVisitor<'_> {
  fn get_cache(&self) -> Vec<CompatBox> {
    self.cache.clone()
  }
}

impl<'a> DestructuringVisitor<'a> {
  pub fn new(source_code: &'a str) -> Self {
    let browser_compat_meta_data: DestructuringBrowserCompatMetadata =
      from_str(include_str!("./destructuring.json")).unwrap();
    Self {
      cache: Vec::new(),
      parent_stack: Vec::new(),
      source_code,
      _phantom: PhantomData {},
      browser_compat_meta_data,
    }
  }

  fn get_source_code(&self, span: Span) -> &str {
    &self.source_code[span.start as usize..span.end as usize]
  }

  fn is_destructuring(&self) -> bool {
    match self.parent_stack.last() {
      Some(AstKind::VariableDeclarator(_))
      | Some(AstKind::AssignmentTargetPattern(_)) => true,
      _ => false,
    }
  }
}

impl<'a> Visit<'a> for DestructuringVisitor<'a> {
  fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.push(kind);
  }

  fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
    self.parent_stack.pop();
  }

  fn visit_object_pattern(&mut self, pat: &oxc_ast::ast::ObjectPattern<'a>) {
    if self.is_destructuring() {
      self.cache.push(CompatBox {
        start: pat.span.start,
        end: pat.span.end,
        code_seg: self.get_source_code(pat.span).to_string(),
        compat: self.browser_compat_meta_data.destructuring.clone(),
      });
    }

    if let Some(_) = pat.rest {
      self.cache.push(CompatBox {
        start: pat.span.start,
        end: pat.span.end,
        code_seg: self.get_source_code(pat.span).to_string(),
        compat: self.browser_compat_meta_data.rest_in_objects.clone(),
      });
    }

    for prop in pat.properties.iter() {
      if prop.computed {
        self.cache.push(CompatBox {
          start: prop.span.start,
          end: prop.span.end,
          code_seg: self.get_source_code(prop.span).to_string(),
          compat: self
            .browser_compat_meta_data
            .computed_property_names
            .clone(),
        });
      }
    }

    walk::walk_object_pattern(self, pat);
  }

  fn visit_array_pattern(&mut self, it: &oxc_ast::ast::ArrayPattern<'a>) {
    if self.is_destructuring() {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.browser_compat_meta_data.destructuring.clone(),
      });
    }
    if let Some(_) = it.rest {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.browser_compat_meta_data.rest_in_arrays.clone(),
      });
    }

    walk::walk_array_pattern(self, it);
  }

  fn visit_array_assignment_target(
    &mut self,
    it: &oxc_ast::ast::ArrayAssignmentTarget<'a>,
  ) {
    if self.is_destructuring() {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.browser_compat_meta_data.destructuring.clone(),
      });
    }

    if let Some(_) = it.rest {
      self.cache.push(CompatBox {
        start: it.span.start,
        end: it.span.end,
        code_seg: self.get_source_code(it.span).to_string(),
        compat: self.browser_compat_meta_data.rest_in_arrays.clone(),
      });
    }
    walk::walk_array_assignment_target(self, it);
  }
}

#[cfg(test)]
mod tests {
  use oxc_allocator::Allocator;

  use crate::syntax::operators_n::t::{t, t_any};

  use super::*;

  #[test]
  fn should_exist_destructuring_of_array() {
    let source_code = r##"const [a, b] = array;"##;
    let allocator = Allocator::default();
    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_destructuring_of_object() {
    let source_code = r##"
const {a, b} = object;
"##;
    let allocator = Allocator::default();
    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_destructuring_of_assignment_1() {
    let source_code = r##"const [a, b] = [1, 2];"##;
    let allocator = Allocator::default();
    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_destructuring_of_assignment_2() {
    let source_code = r##"
let a, b;
[a, b] = array;
"##;
    let allocator = Allocator::default();
    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_not_exist_destructuring_of_computed_property_names() {
    let source_code = r##"
const key = "z";
const { [key]: a } = obj;
"##;

    let allocator = Allocator::default();

    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_not_exist_computed_property_names_of_computed_property_names() {
    let source_code = r##"
const key = "z";
const { [key]: a } = obj;
    "##;

    let allocator = Allocator::default();

    t_any(
      "computed_property_names",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_destructuring_of_rest_in_arrays() {
    let source_code = r##"
const [a, ...b] = array;
"##;

    let allocator = Allocator::default();

    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_arrays_of_rest_in_arrays() {
    let source_code = r##"
    const [a, ...b] = array;
        "##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_arrays",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_arrays_of_only_rest_in_arrays() {
    let source_code = r##"
const [...b] = array;
"##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_arrays",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_destructuring_of_rest_in_objects() {
    let source_code = r##"
const {a, ...b} = object;
"##;

    let allocator = Allocator::default();

    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_objects_of_rest_in_objects() {
    let source_code = r##"
  const {a, ...b} = object;
        "##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_objects",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_destructuring_of_for_of() {
    let source_code = r##"
const people = [];
for (const {
  name: n,
  family: { father: f },
} of people) {
  console.log(`Name: ${n}, Father: ${f}`);
}
"##;
    let allocator = Allocator::default();
    t_any(
      "destructuring",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_objects_of_for_of() {
    let source_code = r##"
const people = [];
for (const {
  name: n,
  family: { father: f,...rest },

} of people) {
  console.log(`Name: ${n}, Father: ${f}`);
}
"##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_objects",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_arrays_of_for_of() {
    let source_code = r##"
const people = [];
for (const {
  name: n,
  family: { father: f },
  user: [...rest]
} of people) {
  console.log(`Name: ${n}, Father: ${f}`);
}
"##;

    let allocator = Allocator::default();

    t_any(
      "rest_in_arrays",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }

  #[test]
  fn should_exist_rest_in_arrays_of_for_of_1() {
    let source_code = r##"
const key = "z";
const people = []
for (const {
  name: n,
  family: { father: f },
  [key]: [...rest]
} of people) {
  console.log(`Name: ${n}, Father: ${f}`);
}
"##;

    let allocator = Allocator::default();

    t_any(
      "computed_property_names",
      source_code,
      &allocator,
      DestructuringVisitor::new,
    );
  }
}
