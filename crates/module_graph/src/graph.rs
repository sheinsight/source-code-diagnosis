use std::{
  collections::HashMap,
  env::current_dir,
  fs::read_to_string,
  path::PathBuf,
  sync::{
    atomic::{AtomicU32, Ordering},
    Arc, Mutex,
  },
};

use anyhow::Result;
use beans::{AstNode, Location, Span};
use bimap::BiMap;
use camino::Utf8PathBuf;
use crossbeam::queue::SegQueue;
use napi_derive::napi;
use oxc_ast::AstKind;
use oxc_resolver::{AliasValue, ResolveOptions, Resolver};
use oxc_span::SourceType;
use rayon::prelude::*;
use serde::Serialize;
use utils::SemanticBuilder;
use wax::{Glob, WalkEntry};

#[napi[object]]
#[derive(Debug, Clone)]
pub struct OptionArgs {
  pub cwd: Option<String>,
  pub pattern: Option<String>,
  pub ignore: Option<Vec<String>>,
  pub alias: Option<HashMap<String, Vec<String>>>,
  pub modules: Option<Vec<String>>,
}

#[napi(object)]
#[derive(Debug, Serialize, Eq, Hash, PartialEq, Clone)]
pub struct Edge {
  pub source: String,
  pub target: String,
  pub ast_node: AstNode,
}

pub struct Graph {
  id_counter: Arc<AtomicU32>,
  bi_map: Arc<Mutex<BiMap<String, String>>>,
  edges: Arc<SegQueue<Edge>>,
}

impl Graph {
  pub fn new(options: Option<OptionArgs>) -> Self {
    Self {
      id_counter: Arc::new(AtomicU32::new(0)),
      bi_map: Arc::new(Mutex::new(BiMap::new())),
      edges: Arc::new(SegQueue::new()),
    }
  }

  fn build_edges(&self, options: Option<OptionArgs>) {
    let alias = match &options {
      Some(OptionArgs {
        alias: Some(alias), ..
      }) => alias.to_owned(),
      _ => HashMap::new(),
    };

    let modules = match &options {
      Some(OptionArgs {
        modules: Some(modules),
        ..
      }) => modules.to_owned(),
      _ => vec!["node_modules".into(), "web_modules".into()],
    };

    let cwd = match &options {
      Some(OptionArgs { cwd: Some(cwd), .. }) => {
        Utf8PathBuf::from(cwd.to_owned()).join("").into_string()
      }
      _ => current_dir().unwrap().display().to_string(),
    };

    let pattern = match &options {
      Some(OptionArgs {
        pattern: Some(pattern),
        ..
      }) => pattern.to_owned(),
      _ => "**/*.{js,ts,jsx,tsx}".to_string(),
    };

    let ignore = match &options {
      Some(OptionArgs {
        ignore: Some(ignore),
        ..
      }) => ignore.iter().map(AsRef::as_ref).collect(),
      _ => vec!["**/node_modules/**", "**/*.d.ts"],
    };

    let resolver_alias = alias
      .into_iter()
      .map(|(key, values)| {
        (key, values.into_iter().map(AliasValue::Path).collect())
      })
      .collect();

    let resolver = Resolver::new(ResolveOptions {
      alias: resolver_alias,
      modules,
      extensions: vec![
        ".ts".into(),
        ".js".into(),
        ".jsx".into(),
        ".tsx".into(),
        ".json".into(),
      ],
      ..ResolveOptions::default()
    });

    let glob = Glob::new(&pattern).unwrap();

    let entries: Vec<_> = glob.walk(&cwd).not(ignore).unwrap().collect();

    entries.par_iter().for_each(|item| {
      let entry = match item {
        Ok(entry) => entry,
        Err(_) => return,
      };

      let path = entry.path();
      if !path.is_file() {
        return;
      }

      let source_code = match read_to_string(&path) {
        Ok(code) => code,
        Err(_) => return,
      };

      let source_type = match SourceType::from_path(&path) {
        Ok(st) => st,
        Err(_) => return,
      };

      let builder = SemanticBuilder::code(&source_code, source_type);
      let handler = builder.build_handler();
      let nodes = handler.semantic.nodes();

      for node in nodes.iter() {
        let import_declaration = match node.kind() {
          AstKind::ImportDeclaration(id) => id,
          _ => continue,
        };

        let value = import_declaration.source.value.to_string();
        let parent = match path.parent() {
          Some(p) => p,
          None => continue,
        };

        let resolved_path = match resolver.resolve(&parent, &value) {
          Ok(rp) => rp,
          Err(_) => continue,
        };

        let (span, loc) = handler.get_node_box(node);

        let source = self.to_relative_path(cwd.as_str(), path.to_path_buf());
        let target = self.to_relative_path(
          cwd.as_str(),
          resolved_path.full_path().to_path_buf(),
        );

        let source_id = self.build_id(&source);
        let target_id = self.build_id(&target);

        let edge = self.build_edge(source_id, target_id, span, loc);
        self.edges.push(edge);
      }
    });
  }

  fn build_id(&self, node: &str) -> String {
    if let Ok(mut bin_map) = self.bi_map.try_lock() {
      if let Some(id) = bin_map.get_by_left(node) {
        id.to_string()
      } else {
        let id = self.id_counter.fetch_add(1, Ordering::SeqCst);
        bin_map.insert(node.to_string(), id.to_string());
        id.to_string()
      }
    } else {
      eprintln!("bi_map lock failed");
      "".to_string()
    }
  }

  fn to_relative_path(&self, cwd: &str, absolute_path_buf: PathBuf) -> String {
    if let Ok(absolute_path) = Utf8PathBuf::from_path_buf(absolute_path_buf) {
      absolute_path.to_string().replace(cwd, "")
    } else {
      "".to_string()
    }
  }

  fn build_edge(
    &self,
    source_id: String,
    target_id: String,
    span: oxc_span::Span,
    loc: Location,
  ) -> Edge {
    Edge {
      source: source_id,
      target: target_id,
      ast_node: AstNode {
        span: Span {
          start: span.start,
          end: span.end,
        },
        loc,
      },
    }
  }
}
