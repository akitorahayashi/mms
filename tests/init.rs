mod common;

use common::TestContext;
use predicates::prelude::*;

#[test]
fn init_creates_empty_catalogue() {
    let ctx = TestContext::new();
    let mut cmd = ctx.cli();
    cmd.arg("init").assert().success().stdout(predicate::str::contains("Created empty"));

    let content = std::fs::read_to_string(ctx.local_mcp_path()).expect("expected local catalogue");
    let json: serde_json::Value = serde_json::from_str(&content).expect("valid json");
    assert_eq!(json["mcpServers"].as_object().unwrap().len(), 0);
}

#[test]
fn init_from_global_populates_catalogue() {
    let ctx = TestContext::new();
    let mut cmd = ctx.cli();
    cmd.arg("init").arg("--from-global").assert().success();

    let local_content = std::fs::read_to_string(ctx.local_mcp_path()).expect("local file");
    let global_content = std::fs::read_to_string(ctx.global_mcp_path()).expect("global file");
    assert_eq!(local_content, global_content);

    let json: serde_json::Value = serde_json::from_str(&local_content).expect("valid json");
    assert!(json["mcpServers"].as_object().unwrap().contains_key("context7"));
}

#[test]
fn version_flag_works() {
    let ctx = TestContext::new();

    ctx.cli()
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}
