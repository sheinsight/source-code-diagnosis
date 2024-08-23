use crate::create_compat;

create_compat! {
  "./while.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_while_statement.push(walk_while_statement);
  },

  walk_while_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::WhileStatement| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::assert_ok_count;

  assert_ok_count! {
    "while",
    setup,
    should_ok_when_use_while_expression,
    r#"
    while (n < 3) {
      n++;
    }
    "#,
    1
  }

  //   use crate::syntax::semantic_tester::SemanticTester;

  //   use super::*;

  //   fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
  //     usage.iter().filter(|item| item.name == "while").count()
  //   }

  //   #[test]
  //   fn should_ok_when_async_generator_function_declaration() {
  //     let mut tester = SemanticTester::from_visitor(WhileVisitor::default());
  //     let usage = tester.analyze(
  //       "
  // let n = 0;

  // while (n < 3) {
  //   n++;
  // }

  // console.log(n);
  // // Expected output: 3

  // ",
  //     );

  //     let count = get_async_function_count(&usage);

  //     assert_eq!(usage.len(), 1);

  //     assert_eq!(count, 1);
  //   }
}
