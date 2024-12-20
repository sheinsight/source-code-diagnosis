use check_filename_case::check_filename_case;
use utils::{GlobArgs, GlobJsArgs};

#[test]
fn test_check_filename_case() -> anyhow::Result<()> {
  let cwd = std::env::current_dir()?.join("tests").join("fixtures");

  let args = GlobArgs::from(GlobJsArgs {
    cwd: cwd.to_string_lossy().to_string(),
    pattern: None,
    ignore: None,
  });

  let res = check_filename_case(args)?;

  assert_eq!(res.len(), 2);

  Ok(())
}
