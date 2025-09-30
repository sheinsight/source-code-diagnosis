use std::{fmt::Display, rc::Rc, sync::Arc};

use beans::{Location, Span};
use napi_derive::napi;
use oxc::diagnostics::Severity;
use oxc_linter::{FixKind, LinterBuilder, Oxlintrc};
use serde::Serialize;
use utils::{GlobErrorHandler, GlobSuccessHandler, glob_by_semantic};

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
  pub loc: Location,
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

  let responses = glob_by_semantic(
    |GlobSuccessHandler {
       path,
       parse,
       semantic,
       relative_path,
       ..
     }| {
      let module_record = Arc::new(oxc_linter::ModuleRecord::new(
        &path,
        &parse.module_record,
        &semantic,
      ));
      let semantic = Rc::new(semantic);

      let source_text = semantic.source_text();

      let diagnostics = linter.run(&path, semantic, module_record);

      let responses = diagnostics
        .into_iter()
        .map(|d| {
          let error = d.error;

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
                .map(|l| {
                  let loc = Location::with_source(
                    &source_text,
                    Span {
                      start: l.offset() as u32,
                      end: l.offset() as u32 + l.len() as u32,
                    },
                  );

                  return CheckOxlintLabelsResponse {
                    span: CheckOxlintLabelsSpan {
                      offset: l.offset() as u32,
                      length: l.len() as u32,
                    },
                    loc,
                  };
                })
                .collect::<Vec<_>>()
            })
            .unwrap_or_default();

          return CheckOxlintResponse {
            file_name: relative_path.clone(),
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
    },
    |GlobErrorHandler { .. }| None,
    &args,
  )?;

  Ok(responses.into_iter().flatten().collect())
}
