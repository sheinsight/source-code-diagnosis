use crate::create_compat;

create_compat! {
  "./for_in.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_in_statement.push(walk_for_in_statement);
  },
  walk_for_in_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForInStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
      "for_in",
      setup,
      should_ok_when_for_in_statement,
      r#"
const object = { a: 1, b: 2, c: 3 };
for (const property in object) {
  console.log(`${property}: ${object[property]}`);
}
"#,
  1
    }
}
