use std::path::Path;

use beans::AstNode;
use oxc_ast::{
  ast::{
    BindingIdentifier, Expression, ImportDeclarationSpecifier,
    JSXOpeningElement,
  },
  AstKind,
};

use oxc_semantic::{Reference, Semantic};
use utils::SemanticHandler;

use crate::response::JSXProps;

use super::response::ModuleMemberUsageResponse;

static ES_NAMESPACE: &str = "ES:NAMESPACE";
static ES_DEFAULT: &str = "ES:DEFAULT";
static SIDE_EFFECTS: &str = "ES:SIDE_EFFECTS";
static EMPTY_SPECIFIERS: &str = "EMPTY_SPECIFIERS";
static DYNAMIC_COMPUTED_MEMBER: &str = "ES:DYNAMIC_COMPUTED_MEMBER";
static UNKNOWN: &str = "UNKNOWN";
static NOT_IMPLEMENTED: &str = "NOT_IMPLEMENTED";
static SPREAD_ATTRIBUTE: &str = "SPREAD_ATTRIBUTE";

pub struct ModuleMemberUsageHandler<'a> {
  npm_name_vec: Vec<String>,
  semantic_handler: SemanticHandler<'a>,
  path_str: String,
}

impl<'a> ModuleMemberUsageHandler<'a> {
  pub fn new<P: AsRef<Path>>(
    npm_name_vec: Vec<String>,
    path: P,
    semantic_handler: SemanticHandler<'a>,
  ) -> Self {
    Self {
      npm_name_vec,
      semantic_handler,
      path_str: path.as_ref().display().to_string(),
    }
  }

  pub fn handle(&self) -> Vec<ModuleMemberUsageResponse> {
    process(
      &self.semantic_handler.semantic,
      &self.npm_name_vec,
      &self.path_str,
    )
  }
}

pub fn process<'a>(
  semantic: &'a Semantic,
  npm_name_vec: &Vec<String>,
  relative_path: &str,
) -> Vec<ModuleMemberUsageResponse> {
  semantic
    .nodes()
    .iter()
    .filter_map(|node| {
      let decl = match node.kind() {
        AstKind::ImportDeclaration(decl) => decl,
        _ => {
          return None;
        }
      };

      let source_name = decl.source.value.as_str().to_string();

      if !npm_name_vec.contains(&source_name) {
        return None;
      }

      let ast_node =
        beans::AstNode::with_source_and_ast_node(semantic.source_text(), node);

      let specifiers = match decl.specifiers {
        Some(ref specs) => specs,
        None => {
          return Some(vec![ModuleMemberUsageResponse {
            lib_name: source_name,
            member_name: SIDE_EFFECTS.to_string(),
            file_path: relative_path.to_string(),
            ast_node,
            props: vec![],
          }]);
        }
      };

      if specifiers.is_empty() {
        return Some(vec![ModuleMemberUsageResponse {
          lib_name: source_name,
          member_name: EMPTY_SPECIFIERS.to_string(),
          file_path: relative_path.to_string(),
          ast_node,
          props: vec![],
        }]);
      }

      let responses =
        each_specifiers(semantic, &source_name, &relative_path, specifiers);

      Some(responses)
    })
    .flatten()
    .collect()
}

fn each_specifiers<'a>(
  semantic: &'a Semantic,
  source_name: &str,
  relative_path: &str,
  specifiers: &oxc_allocator::Vec<ImportDeclarationSpecifier<'a>>,
) -> Vec<ModuleMemberUsageResponse> {
  let responses = specifiers
    .iter()
    .map(|spec| {
      let imported_name = match spec {
        ImportDeclarationSpecifier::ImportSpecifier(import_specifier) => {
          import_specifier.imported.name().as_str()
        }
        ImportDeclarationSpecifier::ImportDefaultSpecifier(_) => ES_DEFAULT,
        ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => ES_NAMESPACE,
      };
      let is_default_specifier = is_default_specifier(spec);
      let references = get_symbol_references(semantic, spec.local());
      let responses = references
        .iter()
        .filter_map(|refer| {
          let reference_node = get_reference_node(semantic, refer);
          let ast_node = AstNode::with_source_and_ast_node(
            semantic.source_text(),
            reference_node,
          );
          let is_in_closing = is_in(&semantic, reference_node, 6, |kind| {
            matches!(kind, AstKind::JSXClosingElement(_))
          })
          .is_some();

          if is_in_closing {
            return None;
          }

          let opening_node = is_in(&semantic, reference_node, 6, |kind| {
            matches!(kind, AstKind::JSXOpeningElement(_))
          });

          if let Some(AstKind::JSXOpeningElement(kind)) =
            opening_node.map(|node| node.kind())
          {
            let name = get_jsx_opening_element_name(
              kind,
              is_default_specifier,
              imported_name,
            );

            let attributes = get_jsx_props(kind);

            return Some(ModuleMemberUsageResponse {
              lib_name: source_name.to_string(),
              member_name: name.to_string(),
              file_path: relative_path.to_string(),
              ast_node: ast_node,
              props: attributes,
            });
          }

          let member_parent_node =
            is_in(&semantic, reference_node, 2, |kind| {
              matches!(kind, AstKind::MemberExpression(_))
            });

          if let Some(AstKind::MemberExpression(kind)) =
            member_parent_node.map(|node| node.kind())
          {
            let name = match kind {
              oxc_ast::ast::MemberExpression::ComputedMemberExpression(
                computed_member_expression,
              ) => match &computed_member_expression.expression {
                Expression::StringLiteral(string_literal) => {
                  string_literal.to_string()
                }
                Expression::TemplateLiteral(_)
                | Expression::Identifier(_)
                | Expression::LogicalExpression(_) => {
                  DYNAMIC_COMPUTED_MEMBER.to_string()
                }
                _ => UNKNOWN.to_string(),
              },
              oxc_ast::ast::MemberExpression::StaticMemberExpression(
                static_member_expression,
              ) => static_member_expression.property.name.to_string(),
              oxc_ast::ast::MemberExpression::PrivateFieldExpression(
                private_field_expression,
              ) => private_field_expression.field.name.to_string(),
            };
            return Some(ModuleMemberUsageResponse {
              lib_name: source_name.to_string(),
              member_name: name.to_string(),
              file_path: relative_path.to_string(),
              ast_node: ast_node,
              props: vec![],
            });
          }

          return Some(ModuleMemberUsageResponse {
            lib_name: source_name.to_string(),
            member_name: imported_name.to_string(),
            file_path: relative_path.to_string(),
            ast_node: ast_node,
            props: vec![],
          });
        })
        .collect::<Vec<_>>();
      return responses;
    })
    .flatten()
    .collect::<Vec<_>>();
  return responses;
}

fn get_jsx_opening_element_name(
  kind: &JSXOpeningElement,
  is_default_specifier: bool,
  imported_name: &str,
) -> String {
  match &kind.name {
    oxc_ast::ast::JSXElementName::Identifier(ident) => ident.name.to_string(),
    oxc_ast::ast::JSXElementName::IdentifierReference(ident_ref) => {
      ident_ref.name.to_string()
    }
    oxc_ast::ast::JSXElementName::NamespacedName(namespace_name) => {
      namespace_name.property.name.to_string()
    }
    oxc_ast::ast::JSXElementName::MemberExpression(jsx_member_expr) => {
      if is_default_specifier {
        imported_name.to_string()
      } else {
        match &jsx_member_expr.object {
          oxc_ast::ast::JSXMemberExpressionObject::IdentifierReference(_) => {
            jsx_member_expr.property.name.to_string()
          }
          oxc_ast::ast::JSXMemberExpressionObject::MemberExpression(
            jsx_member_expression,
          ) => jsx_member_expression.property.name.to_string(),
          oxc_ast::ast::JSXMemberExpressionObject::ThisExpression(_) => {
            "this".to_string()
          }
        }
      }
    }
    oxc_ast::ast::JSXElementName::ThisExpression(_) => "This".to_string(),
  }
}

fn get_symbol_references<'a>(
  semantic: &'a Semantic,
  binding: &BindingIdentifier,
) -> Vec<&'a Reference> {
  if let Some(symbol_id) = binding.symbol_id.get() {
    semantic.symbol_references(symbol_id).into_iter().collect()
  } else {
    vec![]
  }
}

fn get_reference_node<'a>(
  semantic: &'a Semantic,
  reference: &'a Reference,
) -> &'a oxc_semantic::AstNode<'a> {
  semantic.nodes().get_node(reference.node_id())
}

fn is_in<'a>(
  semantic: &'a Semantic,
  node: &'a oxc_semantic::AstNode<'a>,
  max_depth: usize,
  predicate: impl Fn(&AstKind) -> bool,
) -> Option<&'a oxc_semantic::AstNode<'a>> {
  let mut depth = 0;
  let mut current_node_id = node.id();
  while let Some(pn) = semantic.nodes().parent_node(current_node_id) {
    if depth >= max_depth {
      return None;
    }

    if predicate(&pn.kind()) {
      return Some(pn);
    }

    current_node_id = pn.id();
    depth += 1;
  }
  None
}

fn is_default_specifier(specifier: &ImportDeclarationSpecifier) -> bool {
  match specifier {
    ImportDeclarationSpecifier::ImportSpecifier(import_specifier) => {
      !(import_specifier.imported.name() == "default")
    }
    ImportDeclarationSpecifier::ImportDefaultSpecifier(_)
    | ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => false,
  }
}

fn get_jsx_props<'a>(kind: &JSXOpeningElement) -> Vec<JSXProps> {
  let props = kind
    .attributes
    .iter()
    .map(|item| match item {
      oxc_ast::ast::JSXAttributeItem::Attribute(attr) => {
        let namespace = match &attr.name {
          oxc_ast::ast::JSXAttributeName::Identifier(_) => None,
          oxc_ast::ast::JSXAttributeName::NamespacedName(namespace) => {
            Some(namespace.namespace.name.to_string())
          }
        };

        let name = match &attr.name {
          oxc_ast::ast::JSXAttributeName::Identifier(ident) => {
            ident.name.to_string()
          }
          oxc_ast::ast::JSXAttributeName::NamespacedName(namespace) => {
            namespace.property.name.to_string()
          }
        };

        let value = attr.value.as_ref().map(|value| match value {
          oxc_ast::ast::JSXAttributeValue::StringLiteral(string_literal) => {
            string_literal.value.to_string()
          }
          oxc_ast::ast::JSXAttributeValue::ExpressionContainer(_)
          | oxc_ast::ast::JSXAttributeValue::Element(_)
          | oxc_ast::ast::JSXAttributeValue::Fragment(_) => {
            NOT_IMPLEMENTED.to_string()
          }
        });

        return JSXProps {
          namespace,
          name,
          value: value,
        };
      }
      oxc_ast::ast::JSXAttributeItem::SpreadAttribute(_) => {
        return JSXProps {
          namespace: None,
          name: SPREAD_ATTRIBUTE.to_string(),
          value: None,
        }
      }
    })
    .collect();
  props
}

#[cfg(test)]
mod tests {
  use super::ModuleMemberUsageHandler;
  use itertools::Itertools;
  use std::{collections::HashMap, path::PathBuf};
  use utils::SemanticBuilder;

  #[test]
  fn test_jsx_props() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import {Form} from 'shineout';
            function App(){
              return (
                <Form>
                  <Form.Item name="hello"></Form.Item>
                </Form>
              )
            }
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["shineout".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    let grouped = result
      .iter()
      .group_by(|item| item.member_name.clone())
      .into_iter()
      .map(|(key, group)| (key, group.count()))
      .collect::<HashMap<String, usize>>();
    assert_eq!(grouped.keys().len(), 1);
    assert_eq!(grouped["Form"], 2);
    let props_str = result
      .into_iter()
      .filter(|item| !item.props.is_empty())
      .flat_map(|item| item.props)
      .map(|item| item.name)
      .join(",");
    assert_eq!(props_str, "name");
  }

  #[test]
  fn test_form_item() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import {Form} from 'shineout';
            function App(){
              return (
                <Form>
                  <Form.Item></Form.Item>
                </Form>
              )
            }
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["shineout".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 2);
  }

  #[test]
  fn test_computed_member_with_template() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import lib from 'lib';
            const name = 'method';
            lib[`get${name}`]();
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["lib".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn test_jsx_self_closing() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import { Component } from 'lib';
            function App() {
                return <Component />;
            }
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["lib".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn test_multiple_references() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import { method } from 'lib';
            method();
            method();
            method();
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["lib".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 3);
  }

  #[test]
  fn test_empty_import_specifiers() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
          import {} from 'react';
          "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn test_dynamic_computed_member() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
          import lib from 'lib';
          const prop = 'method';
          const result = lib[prop]();
          "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["lib".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 1);
  }

  #[test]
  fn test_nested_member_expression() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import lib from 'lib';
            const result = lib.a.b.c;
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["lib".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "lib");
    assert_eq!(result[0].member_name, "a");
  }

  #[test]
  fn test_mixed_import_types() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import def, { named1, named2 as alias } from 'module';
            def.method();
            named1();
            alias();
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["module".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].member_name, "method");
    assert_eq!(result[1].member_name, "named1");
    assert_eq!(result[2].member_name, "named2");
  }

  #[test]
  fn test_jsx_member_expression_with_alias() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import def, { named1, named2 as alias } from 'module';
            def.method();
            named1();
            function App(){
              return <alias.div>{named1}</alias.div>
            }
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["module".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    let grouped = result
      .into_iter()
      .group_by(|item| item.member_name.clone())
      .into_iter()
      .map(|(key, group)| (key, group.count()))
      .collect::<HashMap<String, usize>>();

    assert_eq!(grouped.keys().len(), 3);
    assert_eq!(grouped["method"], 1);
    assert_eq!(grouped["named1"], 2);
    assert_eq!(grouped["named2"], 1);
  }

  #[test]
  fn test_jsx_nested_member_expression() {
    let file_path_str = PathBuf::from("file_path_str");
    let semantic_builder = SemanticBuilder::js(
      &r#"
            import * as Lib from 'lib';
            function Component() {
                return <Lib.Nested.Component prop={true} />;
            }
            "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["lib".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].member_name, "Nested");
  }

  #[test]
  fn test_private_field_expression() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
      import a from 'a';
      const c = a.#name;
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["a".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "a");
    assert_eq!(result[0].member_name, "name");
  }

  #[test]
  fn test_computed_member_expression() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
      import a from 'a';
      const c = a["b"];
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["a".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "a");
    assert_eq!(result[0].member_name, "b");
  }

  #[test]
  fn test_import_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
      import { useState } from 'react';
      function Component() {
          const [state, setState] = useState(0);
          return <div>{state}</div>;
      }
    "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "useState");
  }

  #[test]
  fn test_import_default_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import React from 'react';
        function Component() {
            return <React.Fragment>Hello</React.Fragment>;
        }
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "Fragment");
  }

  #[test]
  fn test_import_namespace_specifier() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import * as React from 'react';
        function Component() {
            return <React.Fragment>Hello</React.Fragment>;
        }
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "Fragment");
  }

  #[test]
  fn test_side_effects_import() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import 'react';
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();

    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].lib_name, "react");
    assert_eq!(result[0].member_name, "ES:SIDE_EFFECTS");
  }

  #[test]
  fn test_multiple_imports() {
    let file_path_str = PathBuf::from("file_path_str");

    let semantic_builder = SemanticBuilder::js(
      &r#"
        import React, { useState } from 'react';
        import * as ReactDOM from 'react-dom';
        function Component() {
            const [state, setState] = useState(0);
            return <React.Fragment>{state}</React.Fragment>;
        }
        ReactDOM.render(<Component />, document.getElementById('root'));
      "#,
    );
    let semantic_handler = semantic_builder.build_handler();
    let handler = ModuleMemberUsageHandler::new(
      vec!["react".to_string(), "react-dom".to_string()],
      file_path_str,
      semantic_handler.unwrap(),
    );
    let result = handler.handle();

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
