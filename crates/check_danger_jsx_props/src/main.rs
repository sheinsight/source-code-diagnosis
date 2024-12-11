use std::{path::Path, rc::Rc, sync::Arc};

use oxc_allocator::Allocator;
use oxc_diagnostics::DiagnosticService;
use oxc_diagnostics::OxcDiagnostic;
use oxc_linter::{
  AllowWarnDeny, FixKind, LintFilter, LinterBuilder, RuleCategory,
};
use oxc_parser::Parser;
use oxc_semantic::SemanticBuilder;
use oxc_span::GetSpan;
use oxc_span::SourceType;

fn main() -> std::io::Result<()> {
  let path = Path::new("view.jsx");
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

  let semantic = Rc::new(semantic_ret.semantic);

  let linter = LinterBuilder::default()
    .with_filters(vec![
      LintFilter::new(AllowWarnDeny::Deny, "no-debugger").unwrap(),
      LintFilter::new(AllowWarnDeny::Deny, "no-console").unwrap(),
      LintFilter::new(AllowWarnDeny::Deny, "no-commonjs").unwrap(),
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
