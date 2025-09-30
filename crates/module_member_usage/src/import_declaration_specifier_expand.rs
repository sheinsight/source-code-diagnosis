use oxc::ast::ast::ImportDeclarationSpecifier;

use crate::r#const::{ES_DEFAULT, ES_NAMESPACE};

pub trait ImportDeclarationSpecifierExpand<'a> {
  fn is_default(&self) -> bool;
  fn is_namespace(&self) -> bool;
  fn get_imported_name(&self) -> String;
}

impl<'a> ImportDeclarationSpecifierExpand<'a>
  for ImportDeclarationSpecifier<'a>
{
  fn is_default(&self) -> bool {
    match self {
      ImportDeclarationSpecifier::ImportSpecifier(import_specifier) => {
        import_specifier.imported.name() == "default"
      }
      ImportDeclarationSpecifier::ImportDefaultSpecifier(_) => true,
      _ => false,
    }
  }

  fn is_namespace(&self) -> bool {
    match self {
      ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => true,
      _ => false,
    }
  }

  fn get_imported_name(&self) -> String {
    match self {
      ImportDeclarationSpecifier::ImportSpecifier(import_specifier) => {
        import_specifier.imported.name().as_str().to_string()
      }
      ImportDeclarationSpecifier::ImportDefaultSpecifier(_) => {
        ES_DEFAULT.to_string()
      }
      ImportDeclarationSpecifier::ImportNamespaceSpecifier(_) => {
        ES_NAMESPACE.to_string()
      }
    }
  }
}
