use crate::create_compat;

create_compat! {
  "./export_default.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_export_default_declaration.push(walk_export_default_declaration);
  },
  walk_export_default_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::ExportDefaultDeclaration| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "export_default",
    setup,
    should_ok_when_export_default_declaration,
    r#"
    export default 1;
    "#,
    1,
  }
}
