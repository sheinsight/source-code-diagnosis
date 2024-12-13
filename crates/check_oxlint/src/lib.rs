use std::{fmt::Display, rc::Rc, sync::Arc};

use napi_derive::napi;
use oxc_diagnostics::Severity;
use oxc_linter::{FixKind, LinterBuilder, Oxlintrc};
use oxc_semantic::SemanticBuilder;
use serde::Serialize;
use serde_json::json;

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

// #[napi(object, js_name = "CheckOxlintArgs")]
// pub struct CheckOxlintArgs {
//   pub globals: Option<JsObject>,
//   pub rules: Vec<Value>,
// }

impl Display for CheckOxlintResponse {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
  }
}

pub fn check_oxlint(
  rules: Vec<&'static str>,
  args: utils::GlobArgs,
) -> anyhow::Result<Vec<CheckOxlintResponse>> {
  let responses = utils::glob_by(
    |path| {
      let builder = utils::SemanticBuilder::with_file(path).unwrap();

      let parse = oxc_parser::Parser::new(
        &builder.allocator,
        &builder.source_code,
        builder.source_type,
      )
      .parse();

      let program = builder.allocator.alloc(parse.program);

      let semantic = SemanticBuilder::new()
        .with_check_syntax_error(false)
        .with_cfg(true)
        .build(program);

      let module_record = Arc::new(oxc_linter::ModuleRecord::new(
        &path,
        &parse.module_record,
        &semantic.semantic,
      ));

      let rules: serde_json::Value = rules
        .iter()
        .map(|&rule| {
          (
            rule.to_string(),
            serde_json::Value::String("error".to_string()),
          )
        })
        .collect::<serde_json::Map<String, serde_json::Value>>()
        .into();

      let config: Oxlintrc = serde_json::from_value(json!({
          "env":{
            "browser": true,
            "node": true,
            "es6": true,
            "jest": true,
            "shared-node-browser": true
          },
          "rules": rules,
          "globals":{
            "__webpack_public_path__": "readonly",
            "ROOT_PATH": "readonly",
            "__ROOT_SAGA__":"readonly",
            "__ROOT_REDUCER__":"readonly",
            "__ROOT_ROUTE__":"readonly",
            "__ROOT_REDUX_DEVTOOLS__":"readonly"
          }
      }))
      .unwrap();
      let linter = LinterBuilder::from_oxlintrc(true, config)
        .with_fix(FixKind::None)
        .build();

      let semantic = Rc::new(semantic.semantic);

      let diagnostics = linter.run(path, semantic, module_record);

      let responses = diagnostics
        .into_iter()
        .map(|d| {
          let error = d.error;

          let file_name = pathdiff::diff_paths(path, &args.cwd)
            .unwrap()
            .display()
            .to_string();

          let help = error
            .help
            .as_ref()
            .map(|h| h.to_string())
            .unwrap_or("".to_string());
          let url = error
            .url
            .as_ref()
            .map(|u| u.to_string())
            .unwrap_or("".to_string());

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
    },
    &args,
  )?;

  Ok(responses.into_iter().flatten().collect())
}
