mod common;

use common::TestContext;

#[test]
fn add_and_remove_servers_updates_local_catalogue() {
    let ctx = TestContext::new();

    ctx.cli().arg("init").assert().success();

    ctx.cli().args(["add", "context7", "serena"]).assert().success();

    let mut local: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ctx.local_mcp_path()).unwrap()).unwrap();
    let servers = local["mcpServers"].as_object_mut().unwrap();
    assert!(servers.contains_key("context7"));
    assert!(servers.contains_key("serena"));

    ctx.cli().args(["remove", "context7"]).assert().success();

    let local_after: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(ctx.local_mcp_path()).unwrap()).unwrap();
    let servers_after = local_after["mcpServers"].as_object().unwrap();
    assert!(!servers_after.contains_key("context7"));
    assert!(servers_after.contains_key("serena"));
}
