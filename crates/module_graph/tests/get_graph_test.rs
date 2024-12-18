use std::env::current_dir;

use module_graph::{edges::get_graph, model::JsArgs};

#[tokio::test]
async fn test_get_graph() -> anyhow::Result<()> {
  let current = current_dir()?
    .join("tests")
    .join("features")
    .join("get_graph");

  let args = JsArgs {
    cwd: current.to_string_lossy().to_string(),
    pattern: None,
    ignore: None,
    alias: None,
    modules: None,
  };

  let graphics = get_graph(args.into())?;

  let mut edges = graphics
    .graph
    .iter()
    .map(|edges| {
      let source = graphics.dictionaries.get(&edges.source).unwrap();
      let target = graphics.dictionaries.get(&edges.target).unwrap();
      format!("{} -> {}", source, target)
    })
    .collect::<Vec<_>>();

  edges.sort();

  insta::assert_snapshot!(edges.join("\n"));

  Ok(())
}
