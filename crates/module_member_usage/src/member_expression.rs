use oxc_ast::ast::{Expression, ImportDeclarationSpecifier, MemberExpression};

use crate::{
  import_declaration_specifier_expand::ImportDeclarationSpecifierExpand,
  r#const::{DYNAMIC_COMPUTED_MEMBER, UNKNOWN},
};

pub trait MemberExpressionExpand<'a> {
  fn get_member_name(
    &self,
    specifier: &ImportDeclarationSpecifier<'a>,
  ) -> String;
}

impl<'a> MemberExpressionExpand<'a> for MemberExpression<'a> {
  fn get_member_name(
    &self,
    specifier: &ImportDeclarationSpecifier<'a>,
  ) -> String {
    match self {
      MemberExpression::ComputedMemberExpression(
        computed_member_expression,
      ) => match &computed_member_expression.expression {
        Expression::StringLiteral(string_literal) => string_literal.to_string(),
        Expression::TemplateLiteral(_)
        | Expression::Identifier(_)
        | Expression::LogicalExpression(_) => {
          DYNAMIC_COMPUTED_MEMBER.to_string()
        }
        _ => UNKNOWN.to_string(),
      },
      MemberExpression::StaticMemberExpression(static_member_expression) => {
        if specifier.is_default_specifier()
          || specifier.is_namespace_specifier()
        {
          static_member_expression.property.name.to_string()
        } else {
          match &static_member_expression.object {
            Expression::Identifier(_ident) => specifier.get_imported_name(),
            _ => UNKNOWN.to_string(),
          }
        }
      }
      MemberExpression::PrivateFieldExpression(private_field_expression) => {
        private_field_expression.field.name.to_string()
      }
    }
  }
}
