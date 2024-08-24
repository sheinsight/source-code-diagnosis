use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_if_statement.push(walk_if_statement);
  },
  compat {
    name: "if_else",
    description: "if...else 语句",
    tags: ["web-features:snapshot:ecmascript-1"],
    support: {
      chrome: "1",
      chrome_android: "1",
      firefox: "1",
      firefox_android: "1",
      opera: "3",
      opera_android: "10.1",
      safari: "1",
      safari_ios: "1",
      edge: "12",
      oculus: "1",
      node: "0.10.0",
      deno: "1.0",
    }
  },
  walk_if_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::IfStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count!(
    "if_else",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    function testNum(a) {
      let result;
      if (a > 0) {
        result = 'positive';
      } else {
        result = 'NOT positive';
      }
      return result;
    }
    "#,
    1,
  );
}
