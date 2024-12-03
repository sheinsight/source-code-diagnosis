use check_danger_jsx_props::{Args, check_danger_jsx_props};
use std::env::current_dir;

#[test]
fn test_check_syntax() -> anyhow::Result<()> {
  let curr = current_dir()?.join("tests").join("features").join("normal");

  let args = Args {
    cwd: curr.to_string_lossy().to_string(),
    pattern: "*.{js,ts,jsx,tsx}".to_owned(),
    ignore: vec![],
  };

  let mut responses =
    check_danger_jsx_props(vec!["data-if".to_owned()], args).unwrap();

  responses.sort_by(|a, b| a.path.cmp(&b.path));

  assert_eq!(responses.len(), 2);

  responses.iter().for_each(|response| {
    insta::assert_snapshot!(response.to_string());
  });

  Ok(())
}
