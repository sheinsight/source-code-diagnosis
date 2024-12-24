use check_browser_supported::{
  check_browser_supported_with_source_code, target::Target,
};

#[tokio::test]
async fn test_check_browser_supported_with_source_code() -> anyhow::Result<()> {
  let source_code = r#"
            class MyClass {
                #privateField = 42;
                
                constructor() {
                    console.log(this.#privateField);
                }
            }
            
            new MyClass();
        "#
  .to_string();
  let result = check_browser_supported_with_source_code(
    Target {
      chrome: "40".to_string(),
      firefox: Some("45".to_string()),
      safari: Some("9".to_string()),
      edge: Some("13".to_string()),
      node: Some("6.0.0".to_string()),
      // deno: Some("1.0.0".to_string()),
    },
    source_code,
    "test.ts".to_string(),
  )?;

  insta::assert_debug_snapshot!(result);

  Ok(())
}
