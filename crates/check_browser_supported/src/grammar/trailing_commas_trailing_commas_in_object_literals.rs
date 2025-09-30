use oxc::ast::AstKind;

use crate::{compat::get_source_code_segment, create_compat};

create_compat! {
  TrailingCommasInObjectLiterals,
  compat {
      name: "trailing_commas.trailing_commas_in_object_literals",
      description: "Trailing commas in object literals",
      mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas",
      tags: [
          "web-features:snapshot:ecmascript-1"
      ],
      support: {
          chrome: "1",
          chrome_android: "1",
          firefox: "1",
          firefox_android: "1",
          safari: "3",
          safari_ios: "1",
          edge: "12",
          node: "0.10.0",
          deno: "1.0.0",
      }
  },
  fn handle<'a>(&self, source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
      if let AstKind::ObjectExpression(object_expr) = node.kind() {
          let source_segment = get_source_code_segment(source_code, object_expr);
          let trimmed = source_segment.trim_end();
          if let Some(last) = trimmed.split(',').map(|item| item.trim()).last() {
              if last == "}" {
                  return true;
              }
          }
      }
      false
  }
}

#[cfg(test)]
mod tests {
  use super::TrailingCommasInObjectLiterals;
  use crate::assert_source_seg;

  assert_source_seg! {
      should_ok_when_object_has_trailing_comma: {
          setup: TrailingCommasInObjectLiterals::default(),
          source_code: r#"
              const obj = {
                  prop1: 'value1',
                  prop2: 'value2',
                  prop3: 'value3',
              };
          "#,
          eq: [
              r#"{
                  prop1: 'value1',
                  prop2: 'value2',
                  prop3: 'value3',
              }"#,
          ],
          ne: []
      },

      should_not_ok_when_object_has_no_trailing_comma: {
          setup: TrailingCommasInObjectLiterals::default(),
          source_code: r#"
              const obj = {
                  prop1: 'value1',
                  prop2: 'value2',
                  prop3: 'value3'
              };
          "#,
          eq: [],
          ne: [
              r#"{
                  prop1: 'value1',
                  prop2: 'value2',
                  prop3: 'value3'
              }"#,
          ]
      },

      should_ok_with_multiple_objects: {
          setup: TrailingCommasInObjectLiterals::default(),
          source_code: r#"
              const obj1 = {
                  prop1: 'value1',
                  prop2: 'value2',
              };
              const obj2 = {
                  prop3: 'value3',
                  prop4: 'value4',
              };
          "#,
          eq: [
              r#"{
                  prop1: 'value1',
                  prop2: 'value2',
              }"#,
              r#"{
                  prop3: 'value3',
                  prop4: 'value4',
              }"#,
          ],
          ne: []
      }
  }
}
