use std::env::current_dir;

use module_graph::{edges::get_graph, model::JsArgs};

#[tokio::test]
async fn test_get_graph() -> anyhow::Result<()> {
  let current = current_dir()?
    .join("tests")
    .join("fixtures")
    .join("get_graph");

  let args = JsArgs {
    cwd: current.to_string_lossy().to_string(),
    ..Default::default()
  };

  let graphics = get_graph(args.try_into().unwrap())?;

  let mut edges = graphics
    .graph
    .iter()
    .map(|edges| {
      let source = graphics.dictionaries.get(&edges.source_id).unwrap();
      let target = graphics.dictionaries.get(&edges.target_id).unwrap();
      format!("{} -> {}", source, target)
    })
    .collect::<Vec<_>>();

  edges.sort();

  insta::assert_snapshot!(edges.join("\n"));

  Ok(())
}
