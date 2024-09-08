use std::{
  collections::HashMap, error::Error, fs::read_to_string, path::PathBuf,
};

use oxc_allocator::Allocator;
use oxc_ast::{
  ast::{
    ImportDeclarationSpecifier, ImportDefaultSpecifier,
    ImportNamespaceSpecifier, JSXMemberExpression, MemberExpression,
  },
  AstKind,
};
use oxc_parser::Parser;
use oxc_semantic::{AstNode, Reference, Semantic, SemanticBuilder};
use oxc_span::{GetSpan, SourceType, Span};

use crate::utils::{ast_node::Location, find_up_ast_node, offset_to_position};

use super::module_member_usage_location::ModuleMemberUsageLocation;

static ES_NAMESPACE: &str = "ES:NAMESPACE";
static ES_DEFAULT: &str = "ES:DEFAULT";

pub struct SemanticContext {
  pub source_code: String,
  pub source_type: SourceType,
  pub allocator: Allocator,
  pub path: PathBuf,
}

impl SemanticContext {
  pub fn new(path: PathBuf) -> Self {
    let allocator = Allocator::default();
    let source_code = read_to_string(&path).unwrap();
    let source_type = SourceType::from_path(&path).unwrap();
    Self {
      source_code,
      source_type,
      allocator,
      path,
    }
  }

  pub fn build_handler<'a>(
    &'a self,
    npm_name_vec: Vec<String>,
  ) -> ModuleMemberUsageHandler<'a> {
    let ret =
      Parser::new(&self.allocator, &self.source_code, self.source_type).parse();
    let program = self.allocator.alloc(ret.program);
    let semantic = SemanticBuilder::new(&self.source_code, self.source_type)
      .build(program)
      .semantic;
    ModuleMemberUsageHandler::new(semantic, npm_name_vec)
  }
}

pub struct ModuleMemberUsageHandler<'a> {
  npm_name_vec: Vec<String>,
  semantic: Semantic<'a>,
}

impl<'a> ModuleMemberUsageHandler<'a> {
  pub fn new(semantic: Semantic<'a>, npm_name_vec: Vec<String>) -> Self {
    Self {
      semantic,
      npm_name_vec,
    }
  }

  pub fn handle(
    &self,
  ) -> Result<Vec<ModuleMemberUsageLocation>, Box<dyn Error>> {
    let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();

    let nodes = self.semantic.nodes();

    let mut mapper = HashMap::new();

    for node in nodes.iter() {
      if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
        let source_name = import_declaration.source.value.to_string();
        if !self.npm_name_vec.contains(&source_name) {
          continue;
        }

        if let Some(specifiers) = &import_declaration.specifiers {
          for specifier in specifiers {
            match specifier {
              ImportDeclarationSpecifier::ImportSpecifier(import_specifier) => {
                let imported_name =
                  import_specifier.imported.name().to_string();
                let local_name = import_specifier.local.name.to_string();
                mapper.insert(local_name, imported_name);
                if let Some(symbol_id) = import_specifier.local.symbol_id.get()
                {
                  self.semantic.symbol_references(symbol_id).for_each(
                    |reference| {
                      let (reference_node, span, loc) =
                        self.get_position_box(&reference);
                      if let Some(parent) =
                        find_up_ast_node(nodes, &reference_node, 2)
                      {
                        if let AstKind::JSXOpeningElement(_) = parent.kind() {
                          if let AstKind::JSXIdentifier(identifier) =
                            reference_node.kind()
                          {
                            let default_name = "unknown".to_string();
                            let name = mapper
                              .get(&identifier.name.to_string())
                              .unwrap_or(&default_name);
                            inline_usages.push(ModuleMemberUsageLocation {
                              lib_name: source_name.to_string(),
                              member_name: name.to_string(),
                              file_path:
                                "path.to_path_buf().display().to_string()"
                                  .to_string(),
                              ast_node: crate::utils::ast_node::AstNode::new(
                                (span.start, span.end),
                                (loc.start, loc.end),
                              ),
                            });
                          }
                        }
                      }
                    },
                  )
                }
              }
              ImportDeclarationSpecifier::ImportDefaultSpecifier(
                import_default_specifier,
              ) => {
                inline_usages.extend(
                  self
                    .process_default_specifier(
                      import_default_specifier,
                      &source_name,
                    )
                    .into_iter(),
                );
              }
              ImportDeclarationSpecifier::ImportNamespaceSpecifier(
                import_namespace_specifier,
              ) => {
                inline_usages.extend(
                  self
                    .process_namespace_specifier(
                      import_namespace_specifier,
                      &source_name,
                    )
                    .into_iter(),
                );
              }
            }
          }
        }
      }
    }

    Ok(inline_usages.clone())
  }

  fn process_namespace_specifier(
    &self,
    specifier: &ImportNamespaceSpecifier,
    source_name: &str,
  ) -> Vec<ModuleMemberUsageLocation> {
    let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();
    if let Some(symbol_id) = specifier.local.symbol_id.get() {
      self
        .semantic
        .symbol_references(symbol_id)
        .for_each(|reference| {
          let (reference_node, span, loc) = self.get_position_box(&reference);
          if let Some(parent_node) =
            self.semantic.nodes().parent_node(reference_node.id())
          {
            match parent_node.kind() {
              AstKind::MemberExpression(member_expression) => {
                inline_usages.extend(self.process_member_expression(
                  source_name,
                  member_expression,
                  span,
                  loc,
                ));
              }
              AstKind::JSXMemberExpressionObject(_) => {
                if let Some(parent) =
                  self.semantic.nodes().parent_node(parent_node.id())
                {
                  if let AstKind::JSXMemberExpression(member_expression) =
                    parent.kind()
                  {
                    inline_usages.extend(self.process_jsx_member_expression(
                      source_name,
                      member_expression,
                      span,
                      loc,
                    ));
                  }
                }
              }
              _ => {
                inline_usages.push(ModuleMemberUsageLocation {
                  lib_name: source_name.to_string(),
                  member_name: ES_NAMESPACE.to_string(),
                  file_path: "self.semantic.source_path().to_string()"
                    .to_string(),
                  ast_node: crate::utils::ast_node::AstNode::new(
                    (span.start, span.end),
                    (loc.start, loc.end),
                  ),
                });
              }
            }
          }
        });
    }
    inline_usages
  }

  fn process_default_specifier(
    &self,
    specifier: &ImportDefaultSpecifier,
    source_name: &str,
  ) -> Vec<ModuleMemberUsageLocation> {
    let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();
    if let Some(symbol_id) = specifier.local.symbol_id.get() {
      self
        .semantic
        .symbol_references(symbol_id)
        .for_each(|reference| {
          let (reference_node, span, loc) = self.get_position_box(&reference);

          if let Some(parent_node) =
            self.semantic.nodes().parent_node(reference_node.id())
          {
            match parent_node.kind() {
              AstKind::MemberExpression(member_expression) => {
                inline_usages.extend(self.process_member_expression(
                  source_name,
                  member_expression,
                  span,
                  loc,
                ));
              }
              AstKind::JSXMemberExpressionObject(_) => {
                if let Some(parent) =
                  self.semantic.nodes().parent_node(parent_node.id())
                {
                  if let AstKind::JSXMemberExpression(member_expression) =
                    parent.kind()
                  {
                    inline_usages.extend(self.process_jsx_member_expression(
                      source_name,
                      member_expression,
                      span,
                      loc,
                    ));
                  }
                }
              }
              _ => {
                inline_usages.push(ModuleMemberUsageLocation {
                  lib_name: source_name.to_string(),
                  member_name: ES_DEFAULT.to_string(),
                  file_path: "self.semantic.source_path().to_string()"
                    .to_string(),
                  ast_node: crate::utils::ast_node::AstNode::new(
                    (span.start, span.end),
                    (loc.start, loc.end),
                  ),
                });
              }
            }
          }
        });
    }

    inline_usages
  }

  fn process_member_expression(
    &self,
    source_name: &str,
    member_expression: &MemberExpression,
    span: Span,
    loc: Location,
  ) -> Vec<ModuleMemberUsageLocation> {
    let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();
    let property_name = member_expression.static_property_name().unwrap();
    inline_usages.push(ModuleMemberUsageLocation {
      lib_name: source_name.to_string(),
      member_name: property_name.to_string(),
      file_path: "self.semantic.source_path().to_string()".to_string(),
      ast_node: crate::utils::ast_node::AstNode::new(
        (span.start, span.end),
        (loc.start, loc.end),
      ),
    });
    inline_usages
  }

  fn process_jsx_member_expression(
    &self,
    source_name: &str,
    member_expression: &JSXMemberExpression,
    span: Span,
    loc: Location,
  ) -> Vec<ModuleMemberUsageLocation> {
    let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();

    let property_name = member_expression.property.name.to_string();

    inline_usages.push(ModuleMemberUsageLocation {
      lib_name: source_name.to_string(),
      member_name: property_name.to_string(),
      file_path: "self.semantic.source_path().to_string()".to_string(),
      ast_node: crate::utils::ast_node::AstNode::new(
        (span.start, span.end),
        (loc.start, loc.end),
      ),
    });

    inline_usages
  }

  fn get_position_box(
    &self,
    reference: &Reference,
  ) -> (AstNode, Span, Location) {
    let source_code = self.semantic.source_text();
    let reference_node = self.semantic.nodes().get_node(reference.node_id());
    let span = GetSpan::span(&reference_node.kind());
    let start_position =
      offset_to_position(span.start as usize, &source_code).unwrap();
    let end_position =
      offset_to_position(span.end as usize, &source_code).unwrap();
    (
      *reference_node,
      span,
      Location {
        start: start_position,
        end: end_position,
      },
    )
  }
}
