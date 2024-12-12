use std::{fmt::Display, rc::Rc, sync::Arc};

use napi_derive::napi;
use oxc_diagnostics::Severity;
use oxc_linter::{AllowWarnDeny, FixKind, LintFilter, LinterBuilder, Oxlintrc};
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

      // let rule: Vec<&'static str> = vec![
      //   "no-debugger",
      //   "no-console",
      //   "constructor-super",
      //   "for-direction",
      //   "getter-return",
      //   "no-async-promise-executor",
      //   "no-case-declarations",
      //   "no-class-assign",
      //   "no-compare-neg-zero",
      //   "no-cond-assign",
      //   "no-const-assign",
      //   "no-constant-binary-expression",
      //   "no-constant-condition",
      //   "no-control-regex",
      //   "no-delete-var",
      //   "no-dupe-class-members",
      //   "no-dupe-else-if",
      //   "no-dupe-keys",
      //   "no-duplicate-case",
      //   "no-empty",
      //   "no-empty-character-class",
      //   "no-empty-pattern",
      //   "no-ex-assign",
      //   "no-fallthrough",
      //   "no-func-assign",
      //   "no-global-assign",
      //   "no-import-assign",
      //   "no-inner-declarations",
      //   "no-invalid-regexp",
      //   "no-irregular-whitespace",
      //   "no-loss-of-precision",
      //   "no-new-native-nonconstructor",
      //   "no-nonoctal-decimal-escape",
      //   "no-obj-calls",
      //   "no-prototype-builtins",
      //   "no-redeclare",
      //   "no-regex-spaces",
      //   "no-self-assign",
      //   "no-setter-return",
      //   "no-shadow-restricted-names",
      //   "no-sparse-arrays",
      //   "no-this-before-super",
      //   "no-unexpected-multiline",
      //   "no-unreachable",
      //   "no-unsafe-finally",
      //   "no-unsafe-negation",
      //   "no-unsafe-optional-chaining",
      //   "no-unused-labels",
      //   "no-useless-catch",
      //   "no-useless-escape",
      //   "use-isnan",
      //   "valid-typeof",
      // ];

      // let filter: Vec<LintFilter> = rules
      //   .iter()
      //   .map(|r| LintFilter::new(AllowWarnDeny::Deny, r.to_string()).unwrap())
      //   .collect::<Vec<_>>();

      // let rules_str = rules
      //   .iter()
      //   .map(|item| format!(r#""{}": "error""#, item))
      //   .collect::<Vec<_>>()
      //   .join(",");

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
          // "plugins": ["react", "oxc"],
          "rules": rules,
          "globals":{
            "ROOT_REDUCER": "readonly",
            "ROOT_ROUTER": "readonly",
          }
      }))
      .unwrap();
      let linter = LinterBuilder::from_oxlintrc(true, config)
        .with_fix(FixKind::None)
        .build();

      // let linter = LinterBuilder::empty()
      //   .with_filters(filter)
      //   .with_fix(FixKind::None)
      //   .build();

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
