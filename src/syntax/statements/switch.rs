use crate::create_compat;

create_compat! {
  "./switch.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_switch_statement.push(walk_switch_statement);
  },

  walk_switch_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::SwitchStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "switch",
    setup,
    should_ok_when_switch_statement,
    r#"
  const expr = 'Papayas';
  switch (expr) {
    case 'Oranges':
      console.log('Oranges are $0.59 a pound.');
      break;
    case 'Mangoes':
    case 'Papayas':
      console.log('Mangoes and papayas are $2.79 a pound.');
      // Expected output: "Mangoes and papayas are $2.79 a pound."
      break;
    default:
      console.log(`Sorry, we are out of ${expr}.`);
  }
    "#,
    1,
  }
}
