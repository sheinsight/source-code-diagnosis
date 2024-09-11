use std::{fs::read_to_string, path::PathBuf};

use oxc_allocator::Allocator;
use oxc_ast::{ast::BindingIdentifier, AstKind};
use oxc_parser::Parser;
use oxc_semantic::{
  AstNode, Reference, Semantic, SemanticBuilder as OxcSemanticBuilder,
  SemanticBuilderReturn,
};
use oxc_span::{GetSpan, SourceType};
use ropey::Rope;

use super::ast_node::{Location, Position};

pub struct SemanticBuilder {
  pub source_code: String,
  pub source_type: SourceType,
  pub allocator: Allocator,
  pub file_path: Option<PathBuf>,
}

impl SemanticBuilder {
  pub fn new(
    source_code: String,
    source_type: SourceType,
    file_path: Option<PathBuf>,
  ) -> Self {
    let allocator = Allocator::default();
    Self {
      source_code,
      source_type,
      allocator,
      file_path,
    }
  }

  pub fn file(path: PathBuf) -> Self {
    let source_code = read_to_string(&path).unwrap();
    let source_type = SourceType::from_path(&path).unwrap();
    Self::new(source_code, source_type, Some(path))
  }

  pub fn ts(source_code: String) -> Self {
    Self::new(
      source_code,
      SourceType::default()
        .with_module(true)
        .with_typescript(true)
        .with_jsx(true),
      None,
    )
  }

  pub fn js(source_text: String) -> Self {
    Self::new(
      source_text,
      SourceType::default().with_module(true).with_jsx(true),
      None,
    )
  }

  pub fn build(&self) -> Semantic<'_> {
    let semantic_ret = self.build_with_errors();
    if !semantic_ret.errors.is_empty() {
      panic!(
        "Semantic analysis failed:\n\n{}",
        semantic_ret
          .errors
          .iter()
          .map(|e| format!("{e}"))
          .collect::<String>()
      );
    }
    semantic_ret.semantic
  }

  pub fn build_with_errors(&self) -> SemanticBuilderReturn<'_> {
    let parse = oxc_parser::Parser::new(
      &self.allocator,
      &self.source_code,
      self.source_type,
    )
    .parse();

    let program = self.allocator.alloc(parse.program);
    OxcSemanticBuilder::new(&self.source_code, self.source_type)
      .with_check_syntax_error(true)
      .with_trivias(parse.trivias)
      // .with_cfg(self.cfg)
      .build(program)
  }

  pub fn build_handler(&self) -> SemanticHandler {
    let ret =
      Parser::new(&self.allocator, &self.source_code, self.source_type).parse();

    if ret.errors.len() > 0 {
      for err in ret.errors.iter() {
        eprintln!("parse error: {:?}", err);
      }
      panic!("parse error: {:?}", ret.errors);
    }

    let program = self.allocator.alloc(ret.program);

    let semantic = OxcSemanticBuilder::new(&self.source_code, self.source_type)
      .build(program)
      .semantic;
    if let Some(file_path) = &self.file_path {
      let file_path_str = file_path.to_string_lossy().to_string();
      SemanticHandler::new(file_path_str, semantic)
    } else {
      SemanticHandler::new(String::new(), semantic)
    }
  }
}

pub struct SemanticHandler<'a> {
  pub semantic: Semantic<'a>,
  pub file_path_str: Option<String>,
}

impl<'a> SemanticHandler<'a> {
  pub fn new(file_path_str: String, semantic: Semantic<'a>) -> Self {
    Self {
      file_path_str: Some(file_path_str),
      semantic,
    }
  }

  pub fn each_node<F>(&self, mut f: F)
  where
    F: FnMut(&SemanticHandler<'a>, &Semantic<'a>, &AstNode),
  {
    for node in self.semantic.nodes().iter() {
      f(&self, &self.semantic, node);
    }
  }

  pub fn offset_to_position(
    &self,
    offset: usize,
    source_text: &str,
  ) -> Position {
    let rope = Rope::from_str(source_text);
    let line = rope.try_byte_to_line(offset).unwrap_or(0);
    let first_char_of_line = rope.try_line_to_char(line).unwrap_or(0);
    let offset = rope.try_byte_to_char(offset).unwrap_or(0);
    let col = offset - first_char_of_line;
    Position {
      line: line as u32,
      col: col as u32,
    }
  }

  pub fn offset_to_location(
    &self,
    source_text: &str,
    span: oxc_span::Span,
  ) -> Location {
    let start = self.offset_to_position(span.start as usize, source_text);
    let end = self.offset_to_position(span.end as usize, source_text);
    Location { start, end }
  }

  pub fn get_span(&self, ast_node: &AstNode) -> oxc_span::Span {
    GetSpan::span(&ast_node.kind())
  }

  pub fn get_reference_node_box(
    &self,
    reference: &Reference,
  ) -> (&AstNode, oxc_span::Span, Location) {
    let reference_node = self.parse_reference(reference);
    let span = self.get_span(&reference_node);
    let loc = self.offset_to_location(self.semantic.source_text(), span);
    (reference_node, span, loc)
  }

  pub fn get_node_box(&self, node: &AstNode) -> (oxc_span::Span, Location) {
    let span = self.get_span(node);
    let loc = self.offset_to_location(self.semantic.source_text(), span);
    (span, loc)
  }

  pub fn get_symbol_references(
    &self,
    binding: &BindingIdentifier,
  ) -> Vec<&Reference> {
    if let Some(symbol_id) = binding.symbol_id.get() {
      self
        .semantic
        .symbol_references(symbol_id)
        .into_iter()
        .collect()
    } else {
      vec![]
    }
  }

  // 解析 reference
  pub fn parse_reference(&self, reference: &Reference) -> &AstNode {
    let reference_node = self.semantic.nodes().get_node(reference.node_id());
    reference_node
  }

  pub fn find_up_with_kind(
    &self,
    node: &AstNode,
    kind: AstKind,
  ) -> Option<&AstNode> {
    if let Some(parent_node) = self.semantic.nodes().parent_node(node.id()) {
      if matches!(parent_node.kind(), kind) {
        return Some(parent_node);
      } else {
        self.find_up_with_kind(parent_node, kind)
      }
    } else {
      None
    }
  }

  pub fn find_up_with_dep<'b>(
    &'b self,
    node: &'b AstNode,
    mut dep: usize,
  ) -> Option<&'b AstNode> {
    if dep == 0 {
      return Some(node);
    }
    if let Some(parent_node) = self.semantic.nodes().parent_node(node.id()) {
      dep -= 1;
      self.find_up_with_dep(parent_node, dep)
    } else {
      None
    }
  }

  pub fn get_parent_node(&self, node: &AstNode) -> Option<&AstNode> {
    self.semantic.nodes().parent_node(node.id())
  }
}
