use check_syntax::check_syntax;
use std::env::current_dir;
use utils::GlobArgs;

#[test]
fn test_check_syntax() -> anyhow::Result<()> {
  let curr = current_dir()?.join("tests").join("features").join("normal");

  let args = GlobArgs {
    cwd: curr.to_string_lossy().to_string(),
    pattern: "*.{js,ts,jsx,tsx}".to_owned(),
    ignore: vec![],
  };

  let res = check_syntax(args).unwrap();

  assert_eq!(res.len(), 0);

  Ok(())
}

#[test]
fn test_check_syntax_error() -> anyhow::Result<()> {
  let curr = current_dir()?.join("tests").join("features").join("error");

  let args = GlobArgs {
    cwd: curr.to_string_lossy().to_string(),
    pattern: "*.{js,ts,jsx,tsx}".to_owned(),
    ignore: vec![],
  };

  let res = check_syntax(args).unwrap();

  assert_eq!(res.len(), 1);

  Ok(())
}
