use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_labeled_statement.push(walk_labeled_statement);
  },
  compat {
    name: "label",
    description: "标签语句",
    tags: [
      "web-features:snapshot:ecmascript-3"
    ],
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
  walk_labeled_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::LabeledStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "label",
    setup,

    should_ok_when_label_statements,
    r#"
      let str = '';

      loop1: for (let i = 0; i < 5; i++) {
        if (i === 1) {
          continue loop1;
        }
        str = str + i;
      }
    "#,
    1
  }
}
