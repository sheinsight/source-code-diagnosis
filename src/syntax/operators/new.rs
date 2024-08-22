use crate::create_compat;

create_compat! {
  "./new.json",
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_new_expression.push(walk_new_expression);
  },

  walk_new_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::NewExpression| {
    true
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_new",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    function Car(make, model, year) {
      this.make = make;
      this.model = model;
      this.year = year;
    }

    const car1 = new Car('Eagle', 'Talon TSi', 1993);
    "#,
    1,
  }

  //   use crate::syntax::semantic_tester::SemanticTester;

  //   use super::*;

  //   fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
  //     usage
  //       .iter()
  //       .filter(|item| item.name == "operators_new")
  //       .count()
  //   }

  //   #[test]
  //   fn should_ok_when_async_generator_function_declaration() {
  //     let mut tester = SemanticTester::from_visitor(NewVisitor::default());
  //     let usage = tester.analyze(
  //       r#"
  // function Car(make, model, year) {
  //   this.make = make;
  //   this.model = model;
  //   this.year = year;
  // }

  // const car1 = new Car('Eagle', 'Talon TSi', 1993);
  // "#,
  //     );

  //     let count = get_async_function_count(&usage);

  //     assert_eq!(usage.len(), 1);

  //     assert_eq!(count, 1);
  //   }
}
