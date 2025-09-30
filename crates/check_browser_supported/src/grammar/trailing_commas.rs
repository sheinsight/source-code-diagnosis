use oxc::ast::AstKind;
use oxc::span::GetSpan;

use crate::{compat::get_source_code_segment, create_compat};

create_compat! {
  TrailingCommas,
  compat {
    name: "trailing_commas",
    description: "尾随逗号",
    mdn_url: "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Trailing_commas",
    tags: [
      "web-features:snapshot:ecmascript-5"
    ],
    support: {
      chrome: "1",
      chrome_android: "18",
      firefox: "1",
      firefox_android: "4",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0.0",
    }
  },
  fn handle<'a>(&self, source_code: &str, node: &AstNode<'a>, _nodes: &AstNodes<'a>) -> bool {
    match node.kind() {
      AstKind::ArrayExpression(array) => check_trailing_comma(source_code, array, "]"),
      AstKind::ObjectExpression(object) => check_trailing_comma(source_code, object, "}"),
      AstKind::FormalParameters(params) => check_trailing_comma(source_code, params, ")"),
      AstKind::CallExpression(call) => check_trailing_comma(source_code, call, ")"),
      AstKind::ImportDeclaration(import) => check_trailing_comma(source_code, import, "}"),
      AstKind::ExportNamedDeclaration(export) => check_trailing_comma(source_code, export, "}"),
      _ => false,
    }
  }
}

fn check_trailing_comma<N: GetSpan>(
  source_code: &str,
  node: &N,
  a: &str,
) -> bool {
  let source_segment = get_source_code_segment(source_code, node);
  let trimmed = source_segment.trim_end();
  if let Some(last) = trimmed.split(",").map(|item| item.trim()).last() {
    if last.starts_with(a) {
      return true;
    }
  }
  false
}

#[cfg(test)]
mod tests {
  use super::TrailingCommas;
  use crate::assert_source_seg;

  assert_source_seg! {
    should_ok_when_array_expression:{
      setup: TrailingCommas::default(),
      source_code: r#"
        const arr = [
          1,
          2,
          3,
        ];
      "#,
      eq: [
        r#"[
          1,
          2,
          3,
        ]"#,
      ],
      ne: []
    },

    should_ok_when_object_literals:{
      setup: TrailingCommas::default(),
      source_code: r#"
        const object = {
          foo: "bar",
          baz: "qwerty",
          age: 42,
        };
      "#,
      eq: [
        r#"{
          foo: "bar",
          baz: "qwerty",
          age: 42,
        }"#,
      ],
      ne: []
    },

    should_ok_when_function:{
      setup: TrailingCommas::default(),
      source_code: r#"
        function f(p,) {}
      "#,
      eq: [
        r#"(p,)"#,
      ],
      ne: []
    },

    should_ok_when_function_call_expression:{
      setup: TrailingCommas::default(),
      source_code: r#"
        hello(a,b,)
      "#,
      eq: [
        r#"hello(a,b,)"#,
      ],
      ne: []
    },

    should_ok_when_named_import:{
      setup: TrailingCommas::default(),
      source_code: r#"
        import {
          A,
          B,
          C,
        } from "D";
      "#,
      eq: [
        r#"import {
          A,
          B,
          C,
        } from "D";"#,
      ],
      ne: []
    },

    should_ok_when_named_exported:{
      setup: TrailingCommas::default(),
      source_code: r#"
        export {
          A,
          B,
          C,
        };
      "#,
      eq: [
        r#"export {
          A,
          B,
          C,
        };"#,
      ],
      ne: []
    },
  }
}
