use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_export_default_declaration.push(walk_export_default_declaration);
  },
  compat {
    name: "export_default",
    description: "export 语句中的 default 关键字",
    tags: ["web-features:js-modules"],
    support: {
      chrome: "61",
      chrome_android: "61",
      firefox: "60",
      firefox_android: "60",
      opera: "61",
      opera_android: "61",
      safari: "10.1",
      safari_ios: "10.1",
      edge: "16",
      oculus: "61",
      node: "13.2.0",
      deno: "1.0",
    }
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
