use crate::create_compat;

create_compat! {
  "./export_namespace.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_export_all_declaration.push(walk_export_all_declaration);
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
    "export_namespace",
    setup,
    should_ok_when_export_namespace_declaration,
    r#"
    export * as a from './other_module.js';
    "#,
    1,
  }
}
