use std::env::current_dir;

use module_graph::{
  edges::get_graph, model::JsArgs,
  phantom_dependencies::check_phantom_dependencies,
};

#[test]
fn test_check_phantom_dependencies() -> anyhow::Result<()> {
  let current = current_dir()?
    .join("tests")
    .join("fixtures")
    .join("phantom_dependencies");

  let args = JsArgs {
    cwd: current.to_string_lossy().to_string(),
    ..Default::default()
  };

  let graphics = get_graph(args.try_into().unwrap())?;

  let responses =
    check_phantom_dependencies(vec!["react".to_string()], graphics)?;

  let snap = responses
    .graph
    .iter()
    .filter_map(|edge| {
      if let Some(target_metadata) = &edge.target_metadata {
        return Some(format!(
          r#"name: {}  may_be: {}"#,
          responses
            .dictionaries
            .get(&target_metadata.module_id)
            .unwrap(),
          target_metadata.may_be
        ));
      }

      None
    })
    .collect::<Vec<_>>();

  insta::assert_snapshot!("{}", snap.join("\n"));

  Ok(())
}
