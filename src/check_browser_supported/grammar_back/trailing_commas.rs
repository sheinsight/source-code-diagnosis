use regex::Regex;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_array_expression.push(walk_array_expression);
    v.walk_object_expression.push(walk_object_expression);
    v.walk_function.push(walk_function);
    v.walk_call_expression.push(walk_call_expression);
    v.walk_import_declaration.push(walk_import_declaration);
    v.walk_export_named_declaration.push(walk_export_named_declaration);
  },
  compat {
    name: "trailing_commas",
    description: "尾随逗号",
    tags: [],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_array_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ArrayExpression| {
    let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
    if let Ok(regex) = Regex::new(r",\s*\]$") {
      regex.is_match(source_code)
    } else {
      false
    }
  },
  walk_object_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectExpression| {
    let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
    if let Ok(regex) = Regex::new(r",\s*\}$") {
      regex.is_match(source_code)
    } else {
      false
    }
  },
  walk_function,
  |ctx: &mut Context, it: &oxc_ast::ast::Function, _flags: &oxc_semantic::ScopeFlags| {
    let source_code = get_source_code(&ctx.source_code.as_str(), it.params.span);
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      regex.is_match(source_code)
    } else {
      false
    }
  },
  walk_call_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::CallExpression| {
    let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      regex.is_match(source_code)
    } else {
      false
    }
  },
  walk_import_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::ImportDeclaration| {
    let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
    if let Ok(regex) = Regex::new(r##"\{\s*[^}]*,\s*}\s*from\s*['\"][^'\"]*['\"];?"##) {
      regex.is_match(source_code)
    } else {
      false
    }
  },
  walk_export_named_declaration,
  |ctx: &mut Context, it: &oxc_ast::ast::ExportNamedDeclaration| {
    let source_code = get_source_code(&ctx.source_code.as_str(), it.span);
    if let Ok(regex) = Regex::new(r"export\s*\{\s*[^}]*,\s*}") {
      regex.is_match(source_code)
    } else {
      false
    }
  }
}

fn get_source_code(source_code: &str, span: oxc_span::Span) -> &str {
  &source_code[span.start as usize..span.end as usize]
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "trailing_commas",
    setup,

    should_ok_when_array_expression,
    r#"
      const arr = [
        1,
        2,
        3,
      ];
    "#,
    1,

    should_ok_when_object_literals,
    r#"
      const object = {
        foo: "bar",
        baz: "qwerty",
        age: 42,
      };
    "#,
    1,

    should_ok_when_function,
    r#"
      function f(p,) {}
    "#,
    1,

    should_ok_when_class_function,
    r#"
      class C {
        one(a,) {}
        two(a, b,) {}
      }
    "#,
    2,

    should_ok_when_object_function,
    r#"
      const obj = {
        one(a,) {},
        two(a, b,) {}
      };
    "#,
    2,

    should_ok_when_function_call_expression,
    r#"
      hello(a,b,)
    "#,
    1,

    should_ok_when_named_import,
    r#"
      import {
        A,
        B,
        C,
      } from "D";
    "#,
    1,

    should_ok_when_named_exported,
    r#"
      export {
        A,
        B,
        C,
      };

      export { A, B, C, };

      export { A as B, C as D, E as F, };
    "#,
    3,
  }
}
