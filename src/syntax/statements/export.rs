use crate::create_compat;

create_compat! {
  "./export.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_export_named_declaration.push(walk_export_named_declaration);
    v.walk_export_default_declaration.push(walk_export_default_declaration);
    v.walk_export_all_declaration.push(walk_export_all_declaration);
  },
  walk_export_named_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::ExportNamedDeclaration| {
    true
  },

  walk_export_default_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::ExportDefaultDeclaration| {
    true
  },

  walk_export_all_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::ExportAllDeclaration| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "export",
    setup,

    should_ok_when_exported_const_variable,
    r#"export const a = 1;"#,
    1,

    should_ok_when_export_named_declaration,
    r#"export const a = 1;"#,
    1,

    should_ok_when_export_default_declaration,
    r#"export default 1;"#,
    1,

    should_ok_when_export_all_declaration,
    r#"export * from 'a';"#,
    1,
  }
}
