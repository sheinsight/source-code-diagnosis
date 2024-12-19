use std::env::current_dir;

use module_graph::{cycle::check_cycle, edges::get_graph, model::JsArgs};

#[test]
fn test_check_cycle() -> anyhow::Result<()> {
  let current = current_dir()?.join("tests").join("fixtures").join("cycle");

  let args = JsArgs {
    cwd: current.to_string_lossy().to_string(),
    ..Default::default()
  };

  let graphics = get_graph(args.try_into().unwrap())?;

  let responses = check_cycle(graphics)?;

  let mut snap = responses
    .graph
    .iter()
    .map(|edge| {
      let mut x = edge
        .iter()
        .map(|item| {
          let source = responses.dictionaries.get(&item.source_id).unwrap();
          let target = responses.dictionaries.get(&item.target_id).unwrap();
          format!("( {} -> {} )", source, target)
        })
        .collect::<Vec<_>>();
      x.sort();
      x.join("\n")
    })
    .collect::<Vec<_>>();

  snap.sort();

  insta::assert_snapshot!("{}", snap.join("\n -------------------- \n"));

  Ok(())
}
