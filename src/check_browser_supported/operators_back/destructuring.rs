use oxc_ast::AstKind;

use crate::create_compat;

fn is_destructuring(stack: &Vec<AstKind>) -> bool {
  match stack.last() {
    Some(AstKind::VariableDeclarator(_))
    | Some(AstKind::AssignmentTargetPattern(_)) => true,
    _ => false,
  }
}

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
    v.walk_object_pattern.push(walk_object_pattern);
    v.walk_array_pattern.push(walk_array_pattern);
    v.walk_array_assignment_target.push(walk_array_assignment_target);
  },
  compat {
    name: "operators_destructuring",
    description: "解构赋值",
    tags: ["web-features:snapshot:ecmascript-2015"],
    support: {
      chrome: "49",
      chrome_android: "49",
      firefox: "41",
      firefox_android: "41",
      safari: "8",
      safari_ios: "8",
      edge: "14",
      node: "6.0.0",
      deno: "1.0",
    }
  },
  walk_object_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ObjectPattern| {
    is_destructuring(&ctx.stack)
  },

  walk_array_pattern,
  |ctx: &mut Context, it: &oxc_ast::ast::ArrayPattern| {
    is_destructuring(&ctx.stack)
  },

  walk_array_assignment_target,
  |ctx: &mut Context, it: &oxc_ast::ast::ArrayAssignmentTarget| {
    is_destructuring(&ctx.stack)
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_destructuring",
    setup,

    should_ok_when_use_destructuring_of_object,
    r#"
      const {a, b} = object;
    "#,
    1,

    should_ok_when_use_destructuring_of_array,
    r#"
      const [a, b] = array;
    "#,
    1,

    should_ok_when_use_destructuring_of_assignment,
    r#"
      const [a, b] = [1, 2];
    "#,
    1,

    should_ok_when_use_destructuring_of_assignment_2,
    r#"
      let a, b;
      [a, b] = array;
    "#,
    1,

    should_ok_when_use_destructuring_of_computed_property_names,
    r#"
      const key = "z";
      const { [key]: a } = obj;
    "#,
    1,

    should_ok_when_use_destructuring_of_rest_in_arrays,
    r#"
      const [a, ...b] = array;
    "#,
    1,

    should_ok_when_use_destructuring_of_rest_in_objects,
    r#"
      const {a, ...b} = object;
    "#,
    1,

    should_ok_when_use_destructuring_of_for_of,
    r#"
      const people = [];
      for (const {
        name: n,
        family: { father: f },
      } of people) {
        console.log(`Name: ${n}, Father: ${f}`);
      }
    "#,
    1,
  }
}
