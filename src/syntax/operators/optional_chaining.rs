use oxc_ast::ast::Expression;

use crate::create_compat;

create_compat! {
  "./optional_chaining.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_chain_expression.push(walk_chain_expression);
  },
  walk_chain_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::ChainExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_optional_chaining",
    setup,
    should_ok_when_chain_expression_optional_chaining,
    r#"
    const user = {
      name: '1',
      address: {
        city: '2'
      }
    };
    console.log(user?.address?.city);
    console.log(user?.contact?.email);
    "#,
    2
  }

  //   use crate::syntax::semantic_tester::SemanticTester;

  //   use super::*;

  //   fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
  //     usage
  //       .iter()
  //       .filter(|item| item.name == "operators_optional_chaining")
  //       .count()
  //   }

  //   #[test]
  //   fn should_ok_when_async_generator_function_declaration() {
  //     let mut tester =
  //       SemanticTester::from_visitor(OptionalChainingVisitor::default());
  //     let usage = tester.analyze(
  //       "
  // const user = {
  //   name: '1',
  //   address: {
  //     city: '2'
  //   }
  // };
  // console.log(user?.address?.city);
  // console.log(user?.contact?.email);
  // ",
  //     );

  //     let count = get_async_function_count(&usage);

  //     assert_eq!(usage.len(), 2);

  //     assert_eq!(count, 2);
  //   }
}
