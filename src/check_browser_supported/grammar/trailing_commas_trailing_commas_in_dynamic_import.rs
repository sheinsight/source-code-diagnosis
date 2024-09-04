use crate::{
  check_browser_supported::compat::get_source_code_segment, create_compat_2,
};

create_compat_2! {
  TrailingCommasInDynamicImport,
  compat {
      name: "trailing_commas.trailing_commas_in_dynamic_import",
      description: "Trailing commas in dynamic import",
      mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Statements/import#dynamic_imports",
      tags: [
          "web-features:snapshot:ecmascript-2020"
      ],
      support: {
          chrome: "91.0.0",
          chrome_android: "91.0.0",
          firefox: "-1",
          firefox_android: "-1",
          safari: "15.0.0",
          safari_ios: "15.0.0",
          edge: "91.0.0",
          node: "17.5.0",
          deno: "1.17.0",
      }
  },
  fn handle<'a>(&self, source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
      if let AstKind::ImportExpression(import_expr) = node.kind() {
          let source_segment = get_source_code_segment(source_code, import_expr);
          let trimmed = source_segment.trim_end();
          if let Some(last) = trimmed.split(",").map(|item| item.trim()).last() {
              if last == ")" {
                  return true;
              }
          }
      }
      false
  }
}

#[cfg(test)]
mod tests {
  use super::TrailingCommasInDynamicImport;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_import_expression:{
          setup: TrailingCommasInDynamicImport::default(),
          source_code: r#"
              import(
                  './module.js',
              );
          "#,
          eq: [
              r#"import(
                  './module.js',
              )"#,
          ],
          ne: []
      },

      should_ok_when_import_expression_then:{
          setup: TrailingCommasInDynamicImport::default(),
          source_code: r#"
              import(
                  './module.js',
              ).then(module => {

              });
          "#,
          eq: [
              r#"import(
                  './module.js',
              )"#,
          ],
          ne: []
      },

      should_not_ok_when_no_trailing_comma:{
          setup: TrailingCommasInDynamicImport::default(),
          source_code: r#"
              import('./module.js');
          "#,
          eq: [],
          ne: [
              r#"import('./module.js')"#,
          ]
      }
  }
}
