use std::{collections::HashMap, error::Error};

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

use crate::utils::{find_up_ast_node, offset_to_position, Location, Position};

use super::module_member_usage_location::ModuleMemberUsageLocation;

static ES_NAMESPACE: &str = "ES:NAMESPACE";
static ES_DEFAULT: &str = "ES:DEFAULT";

pub struct SemanticContext {
  pub source_code: String,
  pub source_type: SourceType,
  pub allocator: Allocator,
}

impl SemanticContext {
  pub fn new(source_code: String, source_type: SourceType) -> Self {
    let allocator = Allocator::default();
    Self {
      source_code,
      source_type,
      allocator,
    }
  }

  pub fn build_handler(
    &self,
    npm_name_vec: Vec<String>,
  ) -> ModuleMemberUsageHandler {
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
                      let (reference_node, span, start_position, end_position) =
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
                              start: span.start,
                              end: span.end,
                              file_path:
                                "path.to_path_buf().display().to_string()"
                                  .to_string(),
                              loc: Location {
                                start: start_position,
                                end: end_position,
                              },
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
          let (reference_node, span, start_position, end_position) =
            self.get_position_box(&reference);
          if let Some(parent_node) =
            self.semantic.nodes().parent_node(reference_node.id())
          {
            match parent_node.kind() {
              AstKind::MemberExpression(member_expression) => {
                inline_usages.extend(self.process_member_expression(
                  source_name,
                  member_expression,
                  span,
                  start_position,
                  end_position,
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
                      start_position,
                      end_position,
                    ));
                  }
                }
              }
              _ => {
                inline_usages.push(ModuleMemberUsageLocation {
                  lib_name: source_name.to_string(),
                  member_name: ES_NAMESPACE.to_string(),
                  start: span.start,
                  end: span.end,
                  file_path: "self.semantic.source_path().to_string()"
                    .to_string(),
                  loc: Location {
                    start: start_position,
                    end: end_position,
                  },
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
          let (reference_node, span, start_position, end_position) =
            self.get_position_box(&reference);

          if let Some(parent_node) =
            self.semantic.nodes().parent_node(reference_node.id())
          {
            match parent_node.kind() {
              AstKind::MemberExpression(member_expression) => {
                inline_usages.extend(self.process_member_expression(
                  source_name,
                  member_expression,
                  span,
                  start_position,
                  end_position,
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
                      start_position,
                      end_position,
                    ));
                  }
                }
              }
              _ => {
                inline_usages.push(ModuleMemberUsageLocation {
                  lib_name: source_name.to_string(),
                  member_name: ES_DEFAULT.to_string(),
                  start: span.start,
                  end: span.end,
                  file_path: "self.semantic.source_path().to_string()"
                    .to_string(),
                  loc: Location {
                    start: start_position,
                    end: end_position,
                  },
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
    start_position: Position,
    end_position: Position,
  ) -> Vec<ModuleMemberUsageLocation> {
    let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();
    let property_name = member_expression.static_property_name().unwrap();
    inline_usages.push(ModuleMemberUsageLocation {
      lib_name: source_name.to_string(),
      member_name: property_name.to_string(),
      start: span.start,
      end: span.end,
      file_path: "self.semantic.source_path().to_string()".to_string(),
      loc: Location {
        start: start_position,
        end: end_position,
      },
    });
    inline_usages
  }

  fn process_jsx_member_expression(
    &self,
    source_name: &str,
    member_expression: &JSXMemberExpression,
    span: Span,
    start_position: Position,
    end_position: Position,
  ) -> Vec<ModuleMemberUsageLocation> {
    let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();

    let property_name = member_expression.property.name.to_string();

    inline_usages.push(ModuleMemberUsageLocation {
      lib_name: source_name.to_string(),
      member_name: property_name.to_string(),
      start: span.start,
      end: span.end,
      file_path: "self.semantic.source_path().to_string()".to_string(),
      loc: Location {
        start: start_position,
        end: end_position,
      },
    });

    inline_usages
  }

  fn get_position_box(
    &self,
    reference: &Reference,
  ) -> (AstNode, Span, Position, Position) {
    let source_code = self.semantic.source_text();
    let reference_node = self.semantic.nodes().get_node(reference.node_id());
    let span = GetSpan::span(&reference_node.kind());
    let start_position =
      offset_to_position(span.start as usize, &source_code).unwrap();
    let end_position =
      offset_to_position(span.end as usize, &source_code).unwrap();
    (*reference_node, span, start_position, end_position)
  }
}
