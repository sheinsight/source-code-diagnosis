use check_oxlint::check_oxlint;
use std::env::current_dir;

macro_rules! test_oxlint {
  ($name:ident, $rule:expr) => {
    #[test]
    fn $name() -> anyhow::Result<()> {
      let curr = current_dir()?.join("tests").join("features").join($rule);

      let js_args = utils::GlobJsArgs {
        pattern: None,
        ignore: None,
        cwd: curr.to_string_lossy().to_string(),
      };

      let args = utils::GlobArgs::from(js_args);
      let mut responses = check_oxlint(
        format!(
          r#"{{
        "rules":{{
          "{}":"error"
        }}
      }}"#,
          $rule
        ),
        args,
      )?;

      responses.sort_by(|a, b| a.file_name.cmp(&b.file_name));

      responses.iter().for_each(|response| {
        insta::assert_snapshot!(response.to_string());
      });

      Ok(())
    }
  };
}

test_oxlint!(test_debugger, "no-debugger");
test_oxlint!(test_console, "no-console");

test_oxlint!(test_constructor_super, "constructor-super");

test_oxlint!(test_for_direction, "for-direction");

test_oxlint!(test_getter_return, "getter-return");

test_oxlint!(test_no_async_promise_executor, "no-async-promise-executor");

test_oxlint!(
  test_no_constant_binary_expression,
  "no-constant-binary-expression"
);

test_oxlint!(test_no_case_declarations, "no-case-declarations");

test_oxlint!(test_no_class_assign, "no-class-assign");

test_oxlint!(test_no_compare_neg_zero, "no-compare-neg-zero");

test_oxlint!(test_no_cond_assign, "no-cond-assign");

test_oxlint!(test_no_const_assign, "no-const-assign");

test_oxlint!(test_no_constant_condition, "no-constant-condition");

test_oxlint!(test_no_control_regex, "no-control-regex");

test_oxlint!(test_no_delete_var, "no-delete-var");

test_oxlint!(test_no_dupe_class_members, "no-dupe-class-members");

test_oxlint!(test_no_dupe_else_if, "no-dupe-else-if");

test_oxlint!(test_no_dupe_keys, "no-dupe-keys");

test_oxlint!(test_no_duplicate_case, "no-duplicate-case");

test_oxlint!(test_no_new_for_builtins, "unicorn/new-for-builtins");
