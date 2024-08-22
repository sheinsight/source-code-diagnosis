use oxc_syntax::operator::AssignmentOperator;

crate::create_compat! {
  "./unsigned_right_shift_assignment.json",
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_assignment_expression.push(walk_assignment_expression);
  },

  walk_assignment_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::AssignmentExpression| {
    matches!(it.operator, AssignmentOperator::ShiftRightZeroFill)
  }
}

#[cfg(test)]
mod tests {
  use crate::assert_ok_count;

  use super::*;

  assert_ok_count! {
    "operators_unsigned_right_shift_assignment",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
let a = 8;
a >>>= 2;
"#,
    1,
  }

  //   use crate::syntax::semantic_tester::SemanticTester;

  //   use super::*;

  //   fn get_async_function_count(usage: &Vec<CompatBox>) -> usize {
  //     usage
  //       .iter()
  //       .filter(|item| item.name == "operators_unsigned_right_shift_assignment")
  //       .count()
  //   }

  //   #[test]
  //   fn should_ok_when_async_generator_function_declaration() {
  //     let mut tester = SemanticTester::from_visitor(
  //       UnsignedRightShiftAssignmentVisitor::default(),
  //     );
  //     let usage = tester.analyze(
  //       "
  // let a = 8;
  // a >>>= 2;
  // ",
  //     );

  //     let count = get_async_function_count(&usage);

  //     assert_eq!(usage.len(), 1);

  //     assert_eq!(count, 1);
  //   }
}
