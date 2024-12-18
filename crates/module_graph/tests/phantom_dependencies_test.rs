use std::env::current_dir;

use module_graph::{
  edges::get_graph, model::JsArgs,
  phantom_dependencies::check_phantom_dependencies,
};

#[test]
fn test_check_phantom_dependencies() -> anyhow::Result<()> {
  let current = current_dir()?
    .join("tests")
    .join("features")
    .join("phantom_dependencies");

  let args = JsArgs {
    cwd: current.to_string_lossy().to_string(),
    pattern: None,
    ignore: None,
    alias: None,
    modules: None,
  };

  let graphics = get_graph(args.into())?;

  let responses =
    check_phantom_dependencies(vec!["react".to_string()], graphics)?;

  let snap = responses
    .graph
    .iter()
    .filter_map(|edge| {
      if let Some(target_module_name) = &edge.target_module_name {
        return Some(format!(
          "{:?}",
          responses.dictionaries.get(target_module_name).unwrap()
        ));
      }

      None
    })
    .collect::<Vec<_>>();

  insta::assert_snapshot!("{}", snap.join("\n"));

  Ok(())
}
