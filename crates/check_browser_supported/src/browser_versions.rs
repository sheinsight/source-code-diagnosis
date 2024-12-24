use anyhow::{Context, Result};
use browserslist::{resolve, Distrib, Opts};
use std::collections::HashMap;

use crate::target::Target;

#[derive(Debug, Clone)]
pub struct BrowserVersions {
  versions: HashMap<String, Vec<String>>,
}

impl BrowserVersions {
  pub fn new(target: Target) -> Result<Self> {
    let browser_list = Self::get_queries(target)?;
    let mut versions = HashMap::new();
    browser_list.into_iter().for_each(|distrib| {
      versions
        .entry(distrib.name().to_string())
        .or_insert_with(Vec::new)
        .push(distrib.version().to_string());
    });
    Ok(Self { versions })
  }

  fn get_queries(target: Target) -> anyhow::Result<Vec<Distrib>> {
    let mut queries = vec![format!("chrome > {}", target.chrome)];
    if let Some(firefox) = &target.firefox {
      queries.push(format!("firefox > {}", firefox));
    }

    if let Some(safari) = &target.safari {
      queries.push(format!("safari > {}", safari));
    }

    if let Some(edge) = &target.edge {
      queries.push(format!("edge > {}", edge));
    }

    if let Some(node) = &target.node {
      queries.push(format!("node > {}", node));
    }

    // if let Some(deno) = &target.deno {
    //   queries.push(format!("deno > {}", deno));
    // }

    let browser_list =
      resolve(&queries, &Opts::default()).with_context(|| {
        format!(
          "{}:{} - Failed to resolve browser list\nQueries: {:?}",
          file!(),
          line!(),
          queries
        )
      })?;

    Ok(browser_list)
  }

  pub fn contains_version(&self, browser: &str, version: &str) -> bool {
    self
      .versions
      .get(browser)
      .map(|versions| versions.contains(&version.to_string()))
      .unwrap_or(false)
  }
}
