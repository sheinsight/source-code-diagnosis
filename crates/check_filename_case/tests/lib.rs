use check_filename_case::{check_filename_case, Args, JsArgs};

#[test]
fn test_check_filename_case() -> anyhow::Result<()> {
  let cwd = std::env::current_dir()?;

  let args = Args::from(JsArgs {
    cwd: cwd.to_string_lossy().to_string(),
    pattern: None,
    ignore: None,
  });

  let res = check_filename_case(args)?;

  assert_eq!(res.len(), 2);

  Ok(())
}
