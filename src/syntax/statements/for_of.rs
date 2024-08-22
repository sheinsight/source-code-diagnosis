use crate::create_compat;

create_compat! {
  "./for_of.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_of_statement.push(walk_for_of_statement);
  },
  walk_for_of_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForOfStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;
  assert_ok_count! {
    "for_of",
    setup,
    should_ok_when_for_of_statement,
    r#"
        const array1 = ['a', 'b', 'c'];
        for (const element of array1) {
          console.log(element);
        }
      "#,
      1
  }
}
