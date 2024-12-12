use env_logger::Env;
use napi::Result;
use napi_derive::napi;
use utils::{GlobArgs, GlobJsArgs};

#[napi]
pub fn get_graph(
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  let args = module_graph::model::Args::from(args);
  let mut graph = module_graph::graph::Graph::new(args);
  Ok(graph.get_edges())
}

#[napi(ts_args_type = "
  rules: Array<
          | 'no-debugger'
          | 'no-console'
          | 'constructor-super'
          | 'for-direction'
          | 'getter-return'
          | 'no-async-promise-executor'
          | 'no-case-declarations'
          | 'no-class-assign'
          | 'no-compare-neg-zero'
          | 'no-cond-assign'
          | 'no-const-assign'
          | 'no-constant-binary-expression'
          | 'no-constant-condition'
          | 'no-control-regex'
          | 'no-delete-var'
          | 'no-dupe-class-members'
          | 'no-dupe-else-if'
          | 'no-dupe-keys'
          | 'no-duplicate-case'
          | 'no-empty'
          | 'no-empty-character-class'
          | 'no-empty-pattern'
          | 'no-ex-assign'
          | 'no-fallthrough'
          | 'no-func-assign'
          | 'no-global-assign'
          | 'no-import-assign'
          | 'no-inner-declarations'
          | 'no-invalid-regexp'
          | 'no-irregular-whitespace'
          | 'no-loss-of-precision'
          | 'no-new-native-nonconstructor'
          | 'no-nonoctal-decimal-escape'
          | 'no-obj-calls'
          | 'no-prototype-builtins'
          | 'no-redeclare'
          | 'no-regex-spaces'
          | 'no-self-assign'
          | 'no-setter-return'
          | 'no-shadow-restricted-names'
          | 'no-sparse-arrays'
          | 'no-this-before-super'
          | 'no-undef'
          | 'no-unexpected-multiline'
          | 'no-unreachable'
          | 'no-unsafe-finally'
          | 'no-unsafe-negation'
          | 'no-unused-labels'
          | 'no-unused-vars'
          | 'no-useless-catch'
          | 'no-useless-escape'
          | 'no-with'
          | 'require-atomic-updates'
          | 'use-before-define'
          | 'valid-typeof'
          | 'react/jsx-no-comment-textnodes'
          | 'react/jsx-no-duplicate-props'
          | 'react/jsx-no-target-blank'
          | 'react/jsx-no-undef'
          | 'react/no-children-prop'
          | 'react/no-danger-with-children'
          | 'react/no-direct-mutation-state'
          | 'react/no-find-dom-node'
          | 'react/no-is-mounted'
          | 'react/no-render-return-value'
          | 'react/no-string-refs'
          | 'react/no-unescaped-entities'
          | 'react/react-in-jsx-scope'
          | 'react/require-render-return'
          | 'import/export'
          | 'unicorn/new-for-builtins'
          | 'unicorn/no-instanceof-array'
          | 'unicorn/no-invalid-remove-event-listener'
          | 'unicorn/no-thenable'
          | 'unicorn/no-unreadable-array-destructuring'
          | 'unicorn/require-array-join-separator'
          | 'unicorn/require-number-to-fixed-digits-argument'
          | '@ts/no-duplicate-enum-values'
          | '@ts/no-extra-non-null-assertion'
          | '@ts/no-misused-new'
          | '@ts/no-non-null-asserted-optional-chain'
          | '@ts/no-unsafe-declaration-merging'
          | '@ts/no-unsafe-function-type'
          | '@ts/no-wrapper-object-types'
          | '@ts/prefer-namespace-keyword'
        >,
  args: GlobJsArgs
")]
pub fn check_oxlint(
  rules: Vec<&'static str>,
  args: utils::GlobJsArgs,
) -> Result<Vec<check_oxlint::CheckOxlintResponse>> {
  let args = utils::GlobArgs::from(args);
  check_oxlint::check_oxlint(rules, args)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_danger_strings(
  danger_strings: Vec<String>,
  args: GlobJsArgs,
) -> Result<Vec<check_danger_string::CheckDangerResponse>> {
  let args = GlobArgs::from(args);
  check_danger_string::check_danger_strings(danger_strings, args)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_module_member_usage(
  npm_name_vec: Vec<String>,
  args: GlobJsArgs,
) -> Result<Vec<module_member_usage::ModuleMemberUsageResponse>> {
  let args = GlobArgs::from(args);
  module_member_usage::check_module_member_usage(npm_name_vec, args)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_filename_case(
  args: utils::GlobJsArgs,
) -> Result<Vec<check_filename_case::CheckFilenameCaseResponse>> {
  let args = utils::GlobArgs::from(args);
  check_filename_case::check_filename_case(args)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_browser_supported(
  target: check_browser_supported::Target,
  args: utils::GlobJsArgs,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  let args = utils::GlobArgs::from(args);
  check_browser_supported::check_browser_supported(target, args)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_browser_supported_with_source_code(
  target: check_browser_supported::Target,
  source_code: String,
) -> Result<Vec<check_browser_supported::CompatBox>> {
  check_browser_supported::check_browser_supported_with_source_code(
    target,
    source_code,
  )
  .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_phantom_dependencies(
  dependencies: Vec<String>,
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  let _ = env_logger::Builder::from_env(
    Env::default().filter_or("SHINED_LOG", "info"),
  )
  .try_init();
  let args = module_graph::model::Args::from(args);
  let mut graph = module_graph::graph::Graph::new(args);
  graph
    .check_phantom_dependencies(dependencies)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_cycle(
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::GroupGraphics> {
  let args = module_graph::model::Args::from(args);
  let mut graph = module_graph::graph::Graph::new(args);
  let res = graph.check_cycle();
  res.map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_syntax(
  args: utils::GlobJsArgs,
) -> Result<Vec<check_syntax::CheckSyntaxResponse>> {
  let args = utils::GlobArgs::from(args);
  check_syntax::check_syntax(args)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_dependents(
  file: String,
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  let args = module_graph::model::Args::from(args);
  let mut graph = module_graph::graph::Graph::new(args);
  graph
    .check_dependents(file)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_dependencies(
  file: String,
  args: module_graph::model::JsArgs,
) -> Result<module_graph::model::Graphics> {
  let args = module_graph::model::Args::from(args);
  let mut graph = module_graph::graph::Graph::new(args);
  graph
    .check_dependencies(file)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn init_logger() -> napi::Result<()> {
  env_logger::Builder::from_env(Env::default().filter_or("SHINED_LOG", "info"))
    .try_init()
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}

#[napi]
pub fn check_danger_jsx_props(
  danger_jsx_props: Vec<String>,
  args: utils::GlobJsArgs,
) -> Result<Vec<check_danger_jsx_props::CheckDangerJsxPropsResponse>> {
  let args = utils::GlobArgs::from(args);
  check_danger_jsx_props::check_danger_jsx_props(danger_jsx_props, args)
    .map_err(|e| napi::Error::new(napi::Status::GenericFailure, e.to_string()))
}
