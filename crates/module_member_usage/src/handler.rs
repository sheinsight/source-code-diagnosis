use std::{collections::HashMap, error::Error, path::PathBuf};

use beans::{AstNode, Location};
use oxc_ast::{
  ast::{
    ImportDeclarationSpecifier, ImportDefaultSpecifier,
    ImportNamespaceSpecifier, JSXMemberExpression, MemberExpression,
  },
  AstKind,
};

use oxc_span::Span;
use utils::SemanticHandler;

use super::response::Response;

static ES_NAMESPACE: &str = "ES:NAMESPACE";
static ES_DEFAULT: &str = "ES:DEFAULT";
static SIDE_EFFECTS: &str = "ES:SIDE_EFFECTS";
static UNKNOWN: &str = "UNKNOWN";

pub struct ModuleMemberUsageHandler<'a> {
  npm_name_vec: Vec<String>,
  semantic_handler: SemanticHandler<'a>,
  path_str: String,
}

impl<'a> ModuleMemberUsageHandler<'a> {
  pub fn new(
    npm_name_vec: Vec<String>,
    path: PathBuf,
    semantic_handler: SemanticHandler<'a>,
  ) -> Self {
    Self {
      npm_name_vec,
      semantic_handler,
      path_str: path.display().to_string(),
    }
  }

  pub fn handle(&self) -> Result<Vec<Response>, Box<dyn Error>> {
    let mut mapper = HashMap::new();
    let mut inline_usages: Vec<Response> = Vec::new();

    self
      .semantic_handler
      .each_node(|_handler, _semantic, node| {
        if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
          let source_name = import_declaration.source.value.to_string();

          if self.npm_name_vec.contains(&source_name) {
            if let Some(specifiers) = &import_declaration.specifiers {
              if !specifiers.is_empty() {
                for specifier in specifiers {
                  match specifier {
                    ImportDeclarationSpecifier::ImportSpecifier(
                      import_specifier,
                    ) => {
                      let imported_name =
                        import_specifier.imported.name().to_string();
                      let local_name = import_specifier.local.name.to_string();
                      mapper.insert(local_name, imported_name);
                      let references = self
                        .semantic_handler
                        .get_symbol_references(&import_specifier.local);

                      for reference in references {
                        let (reference_node, span, loc) = self
                          .semantic_handler
                          .get_reference_node_box(reference);

                        if let Some(parent) = self
                          .semantic_handler
                          .find_up_with_dep(&reference_node, 2)
                        {
                          if !matches!(
                            parent.kind(),
                            AstKind::JSXClosingElement(_)
                          ) {
                            let name = match reference_node.kind() {
                              AstKind::JSXIdentifier(identifier) => {
                                identifier.name.to_string()
                              }
                              AstKind::IdentifierReference(identifier) => {
                                identifier.name.to_string()
                              }
                              _ => UNKNOWN.to_string(),
                            };
                            let default_name = UNKNOWN.to_string();
                            let name =
                              mapper.get(&name).unwrap_or(&default_name);
                            inline_usages.push(Response {
                              lib_name: source_name.to_string(),
                              member_name: name.to_string(),
                              file_path: self.path_str.clone(),
                              ast_node: AstNode::new(
                                (span.start, span.end),
                                loc,
                              ),
                            });
                          }
                        }
                      }
                    }
                    ImportDeclarationSpecifier::ImportDefaultSpecifier(
                      import_default_specifier,
                    ) => {
                      inline_usages.extend(self.process_default_specifier(
                        import_default_specifier,
                        &source_name,
                      ));
                    }
                    ImportDeclarationSpecifier::ImportNamespaceSpecifier(
                      import_namespace_specifier,
                    ) => {
                      inline_usages.extend(self.process_namespace_specifier(
                        import_namespace_specifier,
                        &source_name,
                      ));
                    }
                  }
                }
              }
            } else {
              let (span, loc) = self.semantic_handler.get_node_box(node);
              inline_usages.push(Response {
                lib_name: source_name.to_string(),
                member_name: SIDE_EFFECTS.to_string(),
                file_path: self.path_str.clone(),
                ast_node: AstNode::new((span.start, span.end), loc),
              });
            }
          }
        }
      });

    Ok(inline_usages.clone())
  }

  fn process_namespace_specifier(
    &self,
    specifier: &ImportNamespaceSpecifier,
    source_name: &str,
  ) -> Vec<Response> {
    let mut inline_usages: Vec<Response> = Vec::new();

    let references = self
      .semantic_handler
      .get_symbol_references(&specifier.local);

    for reference in references {
      let (reference_node, span, loc) =
        self.semantic_handler.get_reference_node_box(reference);

      if let Some(parent_node) =
        self.semantic_handler.get_parent_node(&reference_node)
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
              self.semantic_handler.get_parent_node(&parent_node)
            {
              if let AstKind::JSXMemberExpression(member_expression) =
                parent.kind()
              {
                inline_usages.extend(self.process_jsx_member_expression(
                  source_name,
                  member_expression,
                  &parent,
                  span,
                  loc,
                ));
              }
            }
          }
          _ => {
            inline_usages.push(Response {
              lib_name: source_name.to_string(),
              member_name: ES_NAMESPACE.to_string(),
              file_path: self.path_str.clone(),
              ast_node: AstNode::new((span.start, span.end), loc),
            });
          }
        }
      }
    }

    inline_usages
  }

  fn process_default_specifier(
    &self,
    specifier: &ImportDefaultSpecifier,
    source_name: &str,
  ) -> Vec<Response> {
    let mut inline_usages: Vec<Response> = Vec::new();

    let references = self
      .semantic_handler
      .get_symbol_references(&specifier.local);

    for reference in references {
      let (reference_node, span, loc) =
        self.semantic_handler.get_reference_node_box(reference);
      if let Some(parent_node) =
        self.semantic_handler.get_parent_node(&reference_node)
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
              self.semantic_handler.get_parent_node(&parent_node)
            {
              if let AstKind::JSXMemberExpression(member_expression) =
                parent.kind()
              {
                inline_usages.extend(self.process_jsx_member_expression(
                  source_name,
                  member_expression,
                  &parent,
                  span,
                  loc,
                ));
              }
            }
          }
          _ => {
            inline_usages.push(Response {
              lib_name: source_name.to_string(),
              member_name: ES_DEFAULT.to_string(),
              file_path: self.path_str.clone(),
              ast_node: AstNode::new((span.start, span.end), loc),
            });
          }
        }
      }
    }

    inline_usages
  }

  fn process_member_expression(
    &self,
    source_name: &str,
    member_expression: &MemberExpression,
    span: Span,
    loc: Location,
  ) -> Vec<Response> {
    let mut inline_usages: Vec<Response> = Vec::new();
    let property_name = member_expression.static_property_name().unwrap();
    inline_usages.push(Response {
      lib_name: source_name.to_string(),
      member_name: property_name.to_string(),
      file_path: self.path_str.clone(),
      ast_node: AstNode::new((span.start, span.end), loc),
    });
    inline_usages
  }

  fn process_jsx_member_expression(
    &self,
    source_name: &str,
    member_expression: &JSXMemberExpression,
    member_expression_node: &oxc_semantic::AstNode,
    span: Span,
    loc: Location,
  ) -> Vec<Response> {
    let mut inline_usages: Vec<Response> = Vec::new();

    if let Some(node) = self
      .semantic_handler
      .find_up_with_dep(member_expression_node, 2)
    {
      if let AstKind::JSXClosingElement(_) = node.kind() {
      } else {
        let property_name = member_expression.property.name.to_string();
        inline_usages.push(Response {
          lib_name: source_name.to_string(),
          member_name: property_name.to_string(),
          file_path: self.path_str.clone(),
          ast_node: AstNode::new((span.start, span.end), loc),
        });
      }
    }

    inline_usages
  }
}

#[cfg(test)]
mod tests {
  use super::ModuleMemberUsageHandler;
  use std::path::PathBuf;
  use utils::SemanticBuilder;

  #[test]
  fn test_import_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      r#"
      import { useState } from 'react';
      function Component() {
          const [state, setState] = useState(0);
          return <div>{state}</div>;
      }
    "#
      .to_string(),
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler,
    );
    let result = handler.handle().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "useState");
  }

  #[test]
  fn test_import_default_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      r#"
        import React from 'react';
        function Component() {
            return <React.Fragment>Hello</React.Fragment>;
        }
      "#
      .to_string(),
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler,
    );
    let result = handler.handle().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "Fragment");
  }

  #[test]
  fn test_import_namespace_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      r#"
        import * as React from 'react';
        function Component() {
            return <React.Fragment>Hello</React.Fragment>;
        }
      "#
      .to_string(),
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler,
    );
    let result = handler.handle().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "Fragment");
  }

  #[test]
  fn test_side_effects_import() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      r#"
        import 'react';
      "#
      .to_string(),
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler,
    );
    let result = handler.handle().unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "ES:SIDE_EFFECTS");
  }

  #[test]
  fn test_multiple_imports() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      r#"
        import React, { useState } from 'react';
        import * as ReactDOM from 'react-dom';
        function Component() {
            const [state, setState] = useState(0);
            return <React.Fragment>{state}</React.Fragment>;
        }
        ReactDOM.render(<Component />, document.getElementById('root'));
      "#
      .to_string(),
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string(), "react-dom".to_string()],
      file_path_str,
      semantic_handler,
    );
    let result = handler.handle().unwrap();

    assert_eq!(result.len(), 3);
    assert!(result
      .iter()
      .any(|r| r.lib_name == "react" && r.member_name == "useState"));
    assert!(result
      .iter()
      .any(|r| r.lib_name == "react" && r.member_name == "Fragment"));
    assert!(result
      .iter()
      .any(|r| r.lib_name == "react-dom" && r.member_name == "render"));
  }
}
