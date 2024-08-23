use crate::create_compat;

create_compat! {
  "./label.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_labeled_statement.push(walk_labeled_statement);
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
