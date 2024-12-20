use std::{fmt::Display, fs::read_to_string, rc::Rc, sync::Arc};

use napi_derive::napi;
use oxc_allocator::Allocator;
use oxc_diagnostics::Severity;
use oxc_linter::{FixKind, LinterBuilder, Oxlintrc};
use oxc_semantic::SemanticBuilder;
use serde::Serialize;
use utils::source_type_from_path;

#[napi(object)]
#[derive(Debug, Clone, Serialize)]
pub struct CheckOxlintLabelsSpan {
  pub offset: u32,
  pub length: u32,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize)]
pub struct CheckOxlintLabelsResponse {
  pub span: CheckOxlintLabelsSpan,
}

#[napi(object)]
#[derive(Debug, Clone, Serialize)]
pub struct CheckOxlintResponse {
  pub file_name: String,
  pub help: String,
  pub url: String,
  pub severity: String,
  pub code: String,
  pub message: String,
  pub labels: Vec<CheckOxlintLabelsResponse>,
}

impl Display for CheckOxlintResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
  }
}

pub fn check_oxlint(
  oxlint_config: String,
  args: utils::GlobArgs,
) -> anyhow::Result<Vec<CheckOxlintResponse>> {
  let cx_args: Oxlintrc = serde_json::from_str(&oxlint_config)?;

  let linter = LinterBuilder::from_oxlintrc(true, cx_args.clone())
    .with_fix(FixKind::None)
    .build();

  let responses = utils::glob_by_path(
    |path| {
      if let Ok(source_code) = read_to_string(path) {
        let allocator = Allocator::default();

        let source_type = source_type_from_path(path);

        let parse =
          oxc_parser::Parser::new(&allocator, &source_code, source_type)
            .parse();

        let program = allocator.alloc(parse.program);

        let semantic = SemanticBuilder::new()
          .with_check_syntax_error(false)
          .with_cfg(true)
          .build(program);

        let module_record = Arc::new(oxc_linter::ModuleRecord::new(
          &path,
          &parse.module_record,
          &semantic.semantic,
        ));

        let semantic = Rc::new(semantic.semantic);

        let diagnostics = linter.run(path, semantic, module_record);

        let responses = diagnostics
          .into_iter()
          .map(|d| {
            let error = d.error;

            let file_name = pathdiff::diff_paths(path, &args.cwd)
              .map(|p| p.display().to_string())
              .unwrap_or_else(|| path.display().to_string());

            let file_name = utils::win_path_to_unix(file_name.as_str());

            let help = error.help.as_deref().unwrap_or_default().to_string();

            let url = error.url.as_deref().unwrap_or_default().to_string();

            let severity = match error.severity {
              Severity::Advice => "advice".to_string(),
              Severity::Warning => "warning".to_string(),
              Severity::Error => "error".to_string(),
            };

            let labels = error
              .labels
              .as_ref()
              .map(|v| {
                v.iter()
                  .map(|l| CheckOxlintLabelsResponse {
                    span: CheckOxlintLabelsSpan {
                      offset: l.offset() as u32,
                      length: l.len() as u32,
                    },
                  })
                  .collect::<Vec<_>>()
              })
              .unwrap_or_default();

            return CheckOxlintResponse {
              file_name,
              help,
              url,
              severity,
              code: error.code.to_string(),
              message: error.message.to_string(),
              labels: labels,
            };
          })
          .collect::<Vec<_>>();

        Some(responses)
      } else {
        None
      }
    },
    &args,
  )?;

  Ok(responses.into_iter().flatten().collect())
}
