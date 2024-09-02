use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_export_all_declaration.push(walk_export_all_declaration);
  },
  compat {
    name: "export_namespace",
    description: "export * as namespace",
    tags: [],
    support: {
      chrome: "72",
      chrome_android: "72",
      firefox: "80",
      firefox_android: "80",
      safari: "14.1",
      safari_ios: "14.1",
      edge: "72",
      node: "13.2.0",
      deno: "1.0",
    }
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
