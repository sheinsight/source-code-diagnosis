use crate::create_compat;

create_compat! {
  "./comma.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_sequence_expression.push(walk_sequence_expression);
  },

  walk_sequence_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::SequenceExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_comma",
    setup,
    should_ok_when_use_comma,
    r#"
      let x = 1;
      x = (x++, x);
    "#,
    1
  }
}
