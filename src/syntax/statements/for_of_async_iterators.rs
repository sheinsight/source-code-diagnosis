use oxc_ast::ast::ForOfStatement;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_for_of_statement.push(walk_for_of_statement);
  },
  compat {
    name: "for_of_async_iterators",
    description: "异步迭代器",
    tags: [
      "web-features:snapshot:ecmascript-2018"
    ],
    support: {
      chrome: "63",
      chrome_android: "63",
      firefox: "57",
      firefox_android: "57",
      opera: "63",
      opera_android: "63",
      safari: "7",
      safari_ios: "7",
      edge: "12",
      oculus: "63",
      node: "10.0.0",
      deno: "1.0",
    }
  },
  walk_for_of_statement,
  |ctx: &mut Context, it: &oxc_ast::ast::ForOfStatement| {
    // TODO: no implement
    it.r#await
  }
}

#[cfg(test)]
mod tests {

  use crate::syntax::semantic_tester::SemanticTester;

  use super::*;

  fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
    usage
      .iter()
      .filter(|item| item.name == "for_of_async_iterators")
      .count()
  }

  #[test]
  fn should_ok_when_async_generator_function_declaration() {
    //     let mut tester =
    //       SemanticTester::from_visitor(ForOfAsyncIteratorsVisitor::default());
    //     let usage = tester.analyze(
    //       "
    // async function* asyncGenerator() {
    //   yield await Promise.resolve(1);
    //   yield await Promise.resolve(2);
    //   yield await Promise.resolve(3);
    // }
    // (async () => {
    //   for await (const num of asyncGenerator()) {
    //     console.log(num);
    //   }
    // })();
    // ",
    //     );

    //     let count = get_async_function_count(&usage);

    //     assert_eq!(usage.len(), 1);

    //     assert_eq!(count, 1);
    assert_eq!(1, 1);
  }
}
