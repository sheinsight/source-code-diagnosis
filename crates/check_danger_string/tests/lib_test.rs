use check_danger_string::check_danger_strings;
use utils::GlobJsArgs;

#[test]
fn test_check_danger_string() -> anyhow::Result<()> {
  let result = check_danger_strings(
    vec!["taobao.com".to_string()],
    GlobJsArgs {
      cwd: "tests/fixtures/danger".to_string(),
      pattern: None,
      ignore: None,
    }
    .into(),
  )?;

  insta::assert_debug_snapshot!(result);

  Ok(())
}
