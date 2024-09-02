use oxc_ast::ast::Expression;

use crate::create_compat;

create_compat! {
  setup,
  |v: &mut SyntaxVisitor| {
      v.walk_static_member_expression.push(walk_static_member_expression);
  },
  compat {
    name: "operators_import_meta_resolve",
    description: "<code>import.meta.resolve</code>",
    tags: [],
    support: {
      chrome: "105",
      chrome_android: "105",
      firefox: "106",
      firefox_android: "106",
      safari: "16.4",
      safari_ios: "16.4",
      edge: "105",
      node: "20.6.0",
      deno: "1.24",
    }
  },
  walk_static_member_expression,
  |ctx: &mut Context, it: &oxc_ast::ast::StaticMemberExpression| {
    if let Expression::MetaProperty(meta) = &it.object {
          meta.meta.name == "import" && meta.property.name == "meta" && it.property.name == "resolve"
    } else {
      false
    }
  }
}

#[cfg(test)]
mod tests {
  use super::setup;
  use crate::assert_ok_count;

  assert_ok_count! {
    "operators_import_meta_resolve",
    setup,
    should_ok_when_async_generator_function_declaration,
    r#"
    const relativeURL = import.meta.resolve('./module.js');
    "#,
    1,
  }
}
