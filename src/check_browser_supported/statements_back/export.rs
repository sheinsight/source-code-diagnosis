use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_export_named_declaration.push(walk_export_named_declaration);
    v.walk_export_default_declaration.push(walk_export_default_declaration);
    v.walk_export_all_declaration.push(walk_export_all_declaration);
  },
  compat {
    name: "export",
    description: "The `export` statement is used when creating JavaScript modules to export functions, objects, or primitive values from the module so they can be used by other programs with the `import` statement.",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "61",
      chrome_android: "61",
      firefox: "60",
      firefox_android: "60",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "16",
      node: "13.2.0",
      deno: "1.0",
    }
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
