mod common;

use common::TestContext;

#[test]
fn sync_updates_gemini_and_codex() {
    let ctx = TestContext::new();

    // Prepare local catalogue copied from global.
    ctx.cli().arg("init").arg("--from-global").assert().success();

    // Create placeholder Codex config so sync touches it.
    let codex_dir = ctx.home().join(".codex");
    std::fs::create_dir_all(&codex_dir).unwrap();
    let codex_config = codex_dir.join("config.toml");
    std::fs::write(&codex_config, "[general]\nprofile = \"test\"\n").unwrap();

    ctx.cli().arg("sync").assert().success();

    // Gemini settings should mirror local catalogue.
    let gemini_settings = ctx.work_dir().join(".gemini").join("settings.json");
    let settings_content = std::fs::read_to_string(&gemini_settings).unwrap();
    let settings_json: serde_json::Value = serde_json::from_str(&settings_content).unwrap();
    assert!(settings_json["mcpServers"].as_object().unwrap().contains_key("context7"));

    // Codex configuration should now contain the mcp_servers block.
    let codex_updated = std::fs::read_to_string(&codex_config).unwrap();
    assert!(codex_updated.contains("[mcp_servers.context7]"));
    assert!(codex_updated.contains("command"));
}
