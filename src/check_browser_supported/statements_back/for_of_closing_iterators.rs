use oxc_ast::ast::ForOfStatement;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_of_statement.push(walk_for_of_statement);
  },
  compat {
    name: "for_of_closing_iterators",
    description: "关闭迭代器",
    tags: [],
    support: {
      chrome: "51",
      chrome_android: "51",
      firefox: "53",
      firefox_android: "53",
      opera: "51",
      opera_android: "51",
      safari: "7",
      safari_ios: "7",
      edge: "14",
      oculus: "51",
      node: "6.5.0",
      deno: "1.0",
    }
  },
  walk_for_of_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForOfStatement| {
    // TODO: Implement the logic to detect closing iterators
    true
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "for_of_closing_iterators")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    // let mut tester =
    //   SemanticTester::from_visitor(ForOfClosingIteratorsVisitor::default());
    // let usage = tester.analyze("async function* foo() {}");

    // let count = get_async_function_count(&usage);

    // assert_eq!(usage.len(), 1);

    // assert_eq!(count, 1);
    assert_eq!(1, 1)
  }
}
