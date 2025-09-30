use std::{path::Path, rc::Rc, sync::Arc};

use oxc::allocator::Allocator;
use oxc::diagnostics::DiagnosticService;
use oxc::diagnostics::OxcDiagnostic;
use oxc::parser::Parser;
use oxc::semantic::SemanticBuilder;
use oxc::span::GetSpan;
use oxc::span::SourceType;
use oxc_linter::AllowWarnDeny;
use oxc_linter::{FixKind, LintFilter, LinterBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = Path::new(
    "/Users/10015448/Git/metric-main-server/src/modules/queues/validate-es-lint.service.ts",
  );
  let source_text = std::fs::read_to_string(path)?;

  let allocator = Allocator::default();
  let source_type = SourceType::from_path(path).unwrap();
  let ret = Parser::new(&allocator, &source_text, source_type).parse();

  let semantic_builder = SemanticBuilder::new()
    .with_cfg(true)
    .with_scope_tree_child_ids(true);

  let semantic_ret = semantic_builder.build(&ret.program);
  let module_record = Arc::new(oxc_linter::ModuleRecord::new(
    &path,
    &ret.module_record,
    &semantic_ret.semantic,
  ));

  // println!("{:?}", ret.module_record.requested_modules);

  ret.module_record.import_entries.iter().for_each(|value| {
    println!(
      "--> {:?} {:?} {:?}",
      value.import_name, value.local_name, value.module_request.name
    );
  });

  let semantic = Rc::new(semantic_ret.semantic);

  // let x = LintFilterKind::try_from("no-debugger")?;

  // let x = LintFilter::deny("no-debugger");

  let linter = LinterBuilder::empty()
    .with_filters(vec![
      LintFilter::new(AllowWarnDeny::Deny, "no-debugger")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-console")?,
      LintFilter::new(AllowWarnDeny::Deny, "constructor-super")?,
      LintFilter::new(AllowWarnDeny::Deny, "for-direction")?,
      LintFilter::new(AllowWarnDeny::Deny, "getter-return")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-async-promise-executor")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-case-declarations")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-class-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-compare-neg-zero")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-cond-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-const-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-constant-binary-expression")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-constant-condition")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-control-regex")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-delete-var")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-dupe-class-members")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-dupe-else-if")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-dupe-keys")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-duplicate-case")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-empty")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-empty-character-class")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-empty-pattern")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-ex-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-fallthrough")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-func-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-global-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-import-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-inner-declarations")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-invalid-regexp")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-irregular-whitespace")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-loss-of-precision")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-new-native-nonconstructor")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-nonoctal-decimal-escape")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-obj-calls")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-prototype-builtins")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-redeclare")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-regex-spaces")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-self-assign")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-setter-return")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-shadow-restricted-names")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-sparse-arrays")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-this-before-super")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-unexpected-multiline")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-unreachable")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-unsafe-finally")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-unsafe-negation")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-unsafe-optional-chaining")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-unused-labels")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-useless-catch")?,
      LintFilter::new(AllowWarnDeny::Deny, "no-useless-escape")?,
      LintFilter::new(AllowWarnDeny::Deny, "use-isnan")?,
      LintFilter::new(AllowWarnDeny::Deny, "valid-typeof")?,
      // LintFilter::new(AllowWarnDeny::Deny, "no-commonjs").unwrap(),
      // LintFilter::new(AllowWarnDeny::Warn, RuleCategory::Correctness).unwrap(),
    ])
    .with_fix(FixKind::None)
    .build();

  let diagnostics = linter.run(path, semantic, module_record);

  for message in &diagnostics {
    // println!("{:?}", message.error);
    // let diagnostic = OxcDiagnostic::error(message.error.to_string())
    //   .with_label(message.span())
    //   .with_source_code(source_text.clone());

    eprintln!("{}  {:?}", message.error.code, message.error.severity);
  }

  let diagnostics: Vec<OxcDiagnostic> = diagnostics
    .into_iter()
    .map(|message| {
      OxcDiagnostic::error(message.error.to_string()).with_label(message.span())
    })
    .collect();
  let mut service = DiagnosticService::default();
  let sender = service.sender().clone();
  // 包装诊断信息
  let diagnostic_tuple =
    DiagnosticService::wrap_diagnostics(path, &source_text, diagnostics);

  // 发送并显示诊断信息
  sender.send(Some(diagnostic_tuple)).unwrap();
  sender.send(None).unwrap();
  service.run();

  Ok(())
}
