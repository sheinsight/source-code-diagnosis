use crate::model::Graphics;
use std::collections::HashSet;

pub fn check_phantom_dependencies(
  dependencies: Vec<String>,
  graphics: Graphics,
) -> anyhow::Result<Graphics> {
  let deps: HashSet<_> = dependencies.into_iter().collect();

  let edges = graphics
    .graph
    .into_iter()
    .filter_map(|edge| {
      if let Some(target_metadata) = &edge.target_metadata {
        if let Some(target_module_name) =
          graphics.dictionaries.get(&target_metadata.module_id)
        {
          if !deps.contains(target_module_name) {
            return Some(edge);
          }
        }
      }

      None
    })
    .collect::<Vec<_>>();

  Ok(Graphics {
    dictionaries: graphics.dictionaries,
    graph: edges,
    syntax_errors: graphics.syntax_errors,
  })
}
