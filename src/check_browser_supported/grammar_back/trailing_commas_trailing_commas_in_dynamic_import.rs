use regex::Regex;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_import_expression.push(walk_import_expression);
  },
  compat {
    name: "trailing_commas_trailing_commas_in_dynamic_import",
    description: "动态导入中的尾随逗号",
    tags: [],
    support: {
      chrome: "91",
      chrome_android: "91",
      firefox: "-1",
      firefox_android: "-1",
      safari: "15",
      safari_ios: "15",
      edge: "91",
      node: "17.5.0",
      deno: "1.17",
    }
  },
  walk_import_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ImportExpression| {
    let source_code = &ctx.source_code[it.span.start as usize..it.span.end as usize];
    if let Ok(regex) = Regex::new(r",\s*\)$") {
      regex.is_match(source_code)
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "trailing_commas_trailing_commas_in_dynamic_import",
    setup,

    should_ok_when_import_expression,
    r#"
      import(
        './module.js',
      );
    "#,
    1,

    should_ok_when_import_expression_then,
    r#"
      import(
        './module.js',
      ).then(module => {

      });
    "#,
    1,
  }
}
