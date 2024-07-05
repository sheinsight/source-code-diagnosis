use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use oxc_ast::{
  ast::{Expression, ImportDeclarationSpecifier, JSXElementName, JSXMemberExpressionObject},
  Visit,
};

use super::module_member_usage_location::ModuleMemberUsageLocation;

#[derive(Debug)]
pub struct MemberNameAndPackageName {
  imported_name: String,
  npm_lib_name: String,
}

pub struct ModuleMemberUsageVisitor<'a> {
  pub mapper: HashMap<String, MemberNameAndPackageName>,
  pub npm_libs: Vec<String>,
  pub used: Vec<ModuleMemberUsageLocation>,
  pub file_path: PathBuf,
  _phantom: PhantomData<&'a ()>,
}

static ES_DEFAULT: &str = "ES:DEFAULT";

static ES_NAMESPACE: &str = "ES:NAMESPACE";

impl<'a> ModuleMemberUsageVisitor<'a> {
  pub fn new(file_path: PathBuf, npm_name_vec: Vec<String>) -> Self {
    Self {
      used: Vec::new(),
      mapper: HashMap::new(),
      npm_libs: npm_name_vec,
      file_path: file_path,
      _phantom: PhantomData {},
    }
  }
}

impl<'a> Visit<'a> for ModuleMemberUsageVisitor<'a> {
  fn visit_import_declaration(&mut self, decl: &oxc_ast::ast::ImportDeclaration) {
    let value = decl.source.value.to_string();
    if self.npm_libs.contains(&value) {
      if let Some(specifiers) = &decl.specifiers {
        specifiers.iter().for_each(|specifier| match specifier {
          ImportDeclarationSpecifier::ImportSpecifier(spec) => {
            self.mapper.insert(
              spec.local.name.to_string(),
              MemberNameAndPackageName {
                imported_name: spec.imported.name().to_string(),
                npm_lib_name: value.to_string(),
              },
            );
          }
          ImportDeclarationSpecifier::ImportDefaultSpecifier(spec) => {
            self.mapper.insert(
              spec.local.name.to_string(),
              MemberNameAndPackageName {
                imported_name: ES_DEFAULT.to_string(),
                npm_lib_name: value.to_string(),
              },
            );
          }
          ImportDeclarationSpecifier::ImportNamespaceSpecifier(spec) => {
            self.mapper.insert(
              spec.local.name.to_string(),
              MemberNameAndPackageName {
                imported_name: ES_NAMESPACE.to_string(),
                npm_lib_name: value.to_string(),
              },
            );
          }
        });
      }
    }
  }

  fn visit_identifier_reference(&mut self, ident: &oxc_ast::ast::IdentifierReference<'a>) {
    if let Some(v) = self.mapper.get(&ident.name.to_string()) {
      self.used.push(ModuleMemberUsageLocation {
        lib_name: v.npm_lib_name.to_string(),
        member_name: v.imported_name.to_string(),
        start: ident.span.start,
        end: ident.span.end,
        file_path: self.file_path.display().to_string(),
      });
    }
  }

  fn visit_member_expression(&mut self, expr: &oxc_ast::ast::MemberExpression) {
    if let Expression::Identifier(reference) = expr.object() {
      let name = reference.name.to_string();
      if let Some(v) = self.mapper.get(&name) {
        if v.imported_name == ES_DEFAULT || v.imported_name == ES_NAMESPACE {
          if let Some(property_name) = expr.static_property_name() {
            self.used.push(ModuleMemberUsageLocation {
              lib_name: v.npm_lib_name.to_string(),
              member_name: property_name.to_string(),
              start: reference.span.start,
              end: reference.span.end,
              file_path: self.file_path.display().to_string(),
            })
          }
        } else {
          self.used.push(ModuleMemberUsageLocation {
            lib_name: v.npm_lib_name.to_string(),
            member_name: v.imported_name.to_string(),
            start: reference.span.start,
            end: reference.span.end,
            file_path: self.file_path.display().to_string(),
          })
        }
      }
    }
  }

  fn visit_jsx_opening_element(&mut self, elem: &oxc_ast::ast::JSXOpeningElement<'a>) {
    match &elem.name {
      JSXElementName::MemberExpression(expr) => {
        if let JSXMemberExpressionObject::Identifier(reference) = &expr.object {
          let name = reference.name.to_string();
          if let Some(v) = self.mapper.get(&name) {
            if v.imported_name == ES_DEFAULT || v.imported_name == ES_NAMESPACE {
              self.used.push(ModuleMemberUsageLocation {
                lib_name: v.npm_lib_name.to_string(),
                member_name: expr.property.name.to_string(),
                start: reference.span.start,
                end: reference.span.end,
                file_path: self.file_path.display().to_string(),
              })
            } else {
              self.used.push(ModuleMemberUsageLocation {
                lib_name: v.npm_lib_name.to_string(),
                member_name: v.imported_name.to_string(),
                start: reference.span.start,
                end: reference.span.end,
                file_path: self.file_path.display().to_string(),
              })
            }
          }
        }
      }
      JSXElementName::Identifier(ident) => {
        let name = ident.name.to_string();
        if let Some(v) = self.mapper.get(&name) {
          self.used.push(ModuleMemberUsageLocation {
            lib_name: v.npm_lib_name.to_string(),
            member_name: v.imported_name.to_string(),
            start: ident.span.start,
            end: ident.span.end,
            file_path: self.file_path.display().to_string(),
          })
        }
      }
      JSXElementName::NamespacedName(_) => todo!(),
    }
  }
}
