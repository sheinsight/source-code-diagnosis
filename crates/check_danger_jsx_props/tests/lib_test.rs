use check_danger_jsx_props::check_danger_jsx_props;
use std::env::current_dir;
use utils::{GlobArgs, GlobJsArgs};

#[test]
fn test_check_syntax() -> anyhow::Result<()> {
  let curr = current_dir()?.join("tests").join("features").join("normal");

  let args = GlobArgs {
    cwd: curr.to_string_lossy().to_string(),
    pattern: "*.{js,ts,jsx,tsx}".to_owned(),
    ignore: vec![],
  };

  let mut responses =
    check_danger_jsx_props(vec!["data-if".to_owned()], args).unwrap();

  responses.sort_by(|a, b| a.file_path.cmp(&b.file_path));

  assert_eq!(responses.len(), 2);

  // responses.iter().for_each(|response| {
  //   insta::assert_snapshot!(response.to_string());
  // });

  Ok(())
}

#[test]
fn test_check_syntax_error() -> anyhow::Result<()> {
  let curr = current_dir()?
    .join("tests")
    .join("features")
    .join("syntax_errors");

  let args = GlobJsArgs {
    cwd: curr.to_string_lossy().to_string(),
    pattern: None,
    ignore: None,
  };

  let responses =
    check_danger_jsx_props(vec!["data-if".to_owned()], args.into()).unwrap();

  assert_eq!(responses.len(), 1);

  // insta::assert_snapshot!(responses[0].to_string());

  Ok(())
}
