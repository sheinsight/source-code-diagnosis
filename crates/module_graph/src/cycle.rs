use std::collections::HashMap;

use petgraph::{algo::tarjan_scc, graph::DiGraph, visit::EdgeRef};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::model::{Graphics, GroupGraphics};

fn rebuild_cycle_order(
  graph: &DiGraph<String, &crate::model::Edge>,
  component: &[petgraph::prelude::NodeIndex],
) -> Vec<petgraph::prelude::NodeIndex> {
  let mut ordered = Vec::new();
  let mut current = component[0];

  // 使用边的连接关系重建顺序
  while ordered.len() < component.len() {
    ordered.push(current);

    // 找到下一个在组件中的邻居节点
    for edge in graph.edges(current) {
      let next = edge.target();
      if component.contains(&next) && !ordered.contains(&next) {
        current = next;
        break;
      }
    }
  }

  // 确保循环完整
  if ordered.len() > 1 {
    ordered.push(ordered[0]);
  }

  ordered
}

pub fn check_cycle(graphics: Graphics) -> anyhow::Result<GroupGraphics> {
  let len = graphics.graph.len();
  let mut graph =
    DiGraph::<String, &crate::model::Edge>::with_capacity(len, len);

  let mut node_indices = HashMap::new();

  // 1. 添加所有节点
  for edge in graphics.graph.iter() {
    if !node_indices.contains_key(&edge.source) {
      let idx = graph.add_node(edge.source.clone());
      node_indices.insert(edge.source.clone(), idx);
    }
    if !node_indices.contains_key(&edge.target) {
      let idx = graph.add_node(edge.target.clone());
      node_indices.insert(edge.target.clone(), idx);
    }
  }

  // 2. 添加所有边，同时存储 Edge 引用
  for edge in graphics.graph.iter() {
    let source_idx = node_indices[&edge.source];
    let target_idx = node_indices[&edge.target];
    graph.add_edge(source_idx, target_idx, edge);
  }

  let scc = tarjan_scc(&graph);

  let responses = scc
    .par_iter()
    .filter_map(|component| {
      if component.len() == 1 {
        return None;
      }

      // 直接获取边的数据
      let cycle = component
        .iter()
        .flat_map(|&node| {
          graph
            .edges(node)
            .filter(|edge| component.contains(&edge.target()))
            .map(|edge| (*edge.weight()).clone())
        })
        .collect::<Vec<_>>();

      if cycle.is_empty() {
        eprintln!("Warning: Empty cycle found for SCC of size {}", scc.len());
        return None;
      }

      Some(cycle)
    })
    .collect::<Vec<_>>();

  Ok(GroupGraphics {
    dictionaries: graphics.dictionaries,
    graph: responses,
    invalid_syntax_files: graphics.invalid_syntax_files,
  })
}
