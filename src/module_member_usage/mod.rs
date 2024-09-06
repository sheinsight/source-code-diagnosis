use std::{
  collections::HashMap,
  fs::read_to_string,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use module_member_usage_location::ModuleMemberUsageLocation;
use napi::{Error, Result};
use oxc_allocator::Allocator;
use oxc_ast::{ast::ImportDeclarationSpecifier, AstKind};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::{GetSpan, SourceType};

use crate::{
  oxc_visitor_processor::{oxc_visit_process, Options},
  utils::{find_up_ast_node, offset_to_position, Location, Position},
};

pub mod module_member_usage_location;
// pub mod module_member_usage_visitor;

static ES_NAMESPACE: &str = "ES:NAMESPACE";
static ES_DEFAULT: &str = "ES:DEFAULT";

#[napi]
pub fn get_module_member_usage(
  npm_name_vec: Vec<String>,
  options: Option<Options>,
) -> Result<Vec<ModuleMemberUsageLocation>> {
  let used = Arc::new(Mutex::new(Vec::new()));
  let x = {
    let used = Arc::clone(&used);
    move |path: PathBuf| {
      let source_code = read_to_string(&path)
        .map_err(|err| {
          Error::new(
            napi::Status::GenericFailure,
            format!("Failed to read file: {}: {}", path.display(), err),
          )
        })
        .unwrap();

      let source_type = SourceType::from_path(&path)
        .map_err(|e| Error::new(napi::Status::GenericFailure, e.0.to_string()))
        .unwrap();
      let allocator = Allocator::default();
      let ret = Parser::new(&allocator, &source_code, source_type).parse();
      let program = allocator.alloc(ret.program);
      let semantic = SemanticBuilder::new(&source_code, source_type)
        .build(program)
        .semantic;

      let nodes = semantic.nodes();

      let mut inline_usages: Vec<ModuleMemberUsageLocation> = Vec::new();

      let mut mapper = HashMap::new();

      for node in nodes.iter() {
        if !matches!(node.kind(), AstKind::ImportDeclaration(_)) {
          continue;
        }

        if let AstKind::ImportDeclaration(import_declaration) = node.kind() {
          let source_name = import_declaration.source.value.to_string();
          if !npm_name_vec.contains(&source_name) {
            continue;
          }
          if let Some(specifiers) = &import_declaration.specifiers {
            for specifier in specifiers {
              match specifier {
                ImportDeclarationSpecifier::ImportDefaultSpecifier(_) => {
                  if let Some(symbol_id) = specifier.local().symbol_id.get() {
                    let references =
                      semantic.symbol_references(symbol_id).collect::<Vec<_>>();
                    for reference in references {
                      let reference_node = nodes.get_node(reference.node_id());

                      let span = GetSpan::span(&reference_node.kind());
                      let start_position =
                        offset_to_position(span.start as usize, &source_code)
                          .unwrap();
                      let end_position =
                        offset_to_position(span.end as usize, &source_code)
                          .unwrap();
                      if let Some(parent_node) =
                        nodes.parent_node(reference_node.id())
                      {
                        match parent_node.kind() {
                          AstKind::MemberExpression(member_expression) => {
                            let name =
                              member_expression.static_property_name().unwrap();

                            inline_usages.push(ModuleMemberUsageLocation {
                              lib_name: source_name.to_string(),
                              member_name: name.to_string(),
                              start: span.start,
                              end: span.end,
                              file_path: path
                                .to_path_buf()
                                .display()
                                .to_string(),
                              loc: Location {
                                start: Position {
                                  line: start_position.line,
                                  col: start_position.character,
                                },
                                end: Position {
                                  line: end_position.line,
                                  col: end_position.character,
                                },
                              },
                            });
                          }
                          AstKind::JSXMemberExpressionObject(_) => {
                            if let Some(parent) =
                              nodes.parent_node(parent_node.id())
                            {
                              if let AstKind::JSXMemberExpression(
                                member_expression,
                              ) = parent.kind()
                              {
                                let name =
                                  member_expression.property.name.to_string();

                                inline_usages.push(ModuleMemberUsageLocation {
                                  lib_name: source_name.to_string(),
                                  member_name: name.to_string(),
                                  start: span.start,
                                  end: span.end,
                                  file_path: path
                                    .to_path_buf()
                                    .display()
                                    .to_string(),
                                  loc: Location {
                                    start: Position {
                                      line: start_position.line,
                                      col: start_position.character,
                                    },
                                    end: Position {
                                      line: end_position.line,
                                      col: end_position.character,
                                    },
                                  },
                                });
                              }
                            }
                          }
                          _ => {
                            inline_usages.push(ModuleMemberUsageLocation {
                              lib_name: source_name.to_string(),
                              member_name: ES_DEFAULT.to_string(),
                              start: span.start,
                              end: span.end,
                              file_path: path
                                .to_path_buf()
                                .display()
                                .to_string(),
                              loc: Location {
                                start: Position {
                                  line: start_position.line,
                                  col: start_position.character,
                                },
                                end: Position {
                                  line: end_position.line,
                                  col: end_position.character,
                                },
                              },
                            });
                          }
                        }
                      }
                    }
                  }
                }
                ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => {
                  if let Some(symbol_id) = specifier.local().symbol_id.get() {
                    let references =
                      semantic.symbol_references(symbol_id).collect::<Vec<_>>();
                    for reference in references {
                      let reference_node = nodes.get_node(reference.node_id());
                      let span = GetSpan::span(&reference_node.kind());
                      let start_position =
                        offset_to_position(span.start as usize, &source_code)
                          .unwrap();
                      let end_position =
                        offset_to_position(span.end as usize, &source_code)
                          .unwrap();
                      if let Some(parent_node) =
                        nodes.parent_node(reference_node.id())
                      {
                        match parent_node.kind() {
                          AstKind::MemberExpression(member_expression) => {
                            let name =
                              member_expression.static_property_name().unwrap();
                            inline_usages.push(ModuleMemberUsageLocation {
                              lib_name: source_name.to_string(),
                              member_name: name.to_string(),
                              start: span.start,
                              end: span.end,
                              file_path: path
                                .to_path_buf()
                                .display()
                                .to_string(),
                              loc: Location {
                                start: Position {
                                  line: start_position.line,
                                  col: start_position.character,
                                },
                                end: Position {
                                  line: end_position.line,
                                  col: end_position.character,
                                },
                              },
                            });
                          }
                          AstKind::JSXMemberExpressionObject(_) => {
                            if let Some(parent) =
                              nodes.parent_node(parent_node.id())
                            {
                              if let AstKind::JSXMemberExpression(
                                member_expression,
                              ) = parent.kind()
                              {
                                let name =
                                  member_expression.property.name.to_string();
                                inline_usages.push(ModuleMemberUsageLocation {
                                  lib_name: source_name.to_string(),
                                  member_name: name.to_string(),
                                  start: span.start,
                                  end: span.end,
                                  file_path: path
                                    .to_path_buf()
                                    .display()
                                    .to_string(),
                                  loc: Location {
                                    start: Position {
                                      line: start_position.line,
                                      col: start_position.character,
                                    },
                                    end: Position {
                                      line: end_position.line,
                                      col: end_position.character,
                                    },
                                  },
                                });
                              }
                            }
                          }
                          _ => {
                            inline_usages.push(ModuleMemberUsageLocation {
                              lib_name: source_name.to_string(),
                              member_name: ES_NAMESPACE.to_string(),
                              start: span.start,
                              end: span.end,
                              file_path: path
                                .to_path_buf()
                                .display()
                                .to_string(),
                              loc: Location {
                                start: Position { line: 0, col: 0 },
                                end: Position { line: 0, col: 0 },
                              },
                            });
                          }
                        }
                      }
                    }
                  }
                }
                ImportDeclarationSpecifier::ImportSpecifier(
                  import_specifier,
                ) => {
                  let imported_name =
                    import_specifier.imported.name().to_string();
                  let local_name = import_specifier.local.name.to_string();

                  mapper.insert(local_name, imported_name);

                  if let Some(symbol_id) =
                    import_specifier.local.symbol_id.get()
                  {
                    let references =
                      semantic.symbol_references(symbol_id).collect::<Vec<_>>();
                    for reference in references {
                      let reference_node = nodes.get_node(reference.node_id());
                      let span = GetSpan::span(&reference_node.kind());
                      let start_position =
                        offset_to_position(span.start as usize, &source_code)
                          .unwrap();
                      let end_position =
                        offset_to_position(span.end as usize, &source_code)
                          .unwrap();

                      if let Some(parent) =
                        find_up_ast_node(nodes, reference_node, 2)
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
                              file_path: path
                                .to_path_buf()
                                .display()
                                .to_string(),
                              loc: Location {
                                start: Position {
                                  line: start_position.line,
                                  col: start_position.character,
                                },
                                end: Position {
                                  line: end_position.line,
                                  col: end_position.character,
                                },
                              },
                            });
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
          source_name
        } else {
          continue;
        };
      }

      let mut used = used.lock().unwrap();
      used.extend(inline_usages);
    }
  };
  oxc_visit_process(x, options)?;

  let used = Arc::try_unwrap(used)
    .ok()
    .expect("Arc has more than one strong reference")
    .into_inner()
    .expect("Mutex cannot be locked");

  println!("{:?}", used.len());

  Ok(used)
}
