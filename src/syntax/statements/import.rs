use crate::create_compat;

create_compat! {
  "./import.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_import_declaration.push(walk_import_declaration);
  },

  walk_import_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::ImportDeclaration| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "import",
    setup,

    should_ok_when_import_default_specifier,
    r#"
    import defaultExport from 'module-name';
    "#,
    1,

    should_ok_when_import_namespace_specifier,
    r#"
    import * as name from 'module-name';
    "#,
    1,

    should_ok_when_import_specifier,
    r#"
    import { export1 } from 'module-name';
    "#,
    1,

    should_ok_when_import_alias_specifier,
    r#"
    import { export1 as alias1 } from 'module-name';
    "#,
    1,

    should_ok_when_import_declaration,
    r#"
    import 'module-name';
    "#,
    1,
  }
}
