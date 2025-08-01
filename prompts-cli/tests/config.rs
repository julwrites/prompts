use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs;
use prompts_cli::{Prompt, Prompts, storage::JsonStorage};
use toml::Value;

#[tokio::test]
async fn test_cli_config_file() -> anyhow::Result<()> {
    let config_dir = tempdir()?;
    let config_path = config_dir.path().join("config.toml");

    // Create a dedicated temporary directory for prompts storage
    let prompts_storage_dir = tempdir()?;
    let prompts_storage_path = prompts_storage_dir.path();

    let mut config = toml::map::Map::new();
    let mut storage_config = toml::map::Map::new();
    storage_config.insert(
        "path".to_string(),
        Value::String(prompts_storage_path.to_string_lossy().into_owned()),
    );
    config.insert("storage".to_string(), Value::Table(storage_config));

    let config_content = toml::to_string(&config)?;
    fs::write(&config_path, config_content)?;

    // Add a prompt using the prompts_api directly to the configured storage path
    let storage = JsonStorage::new(Some(prompts_storage_path.to_path_buf()))?;
    let prompts_api = Prompts::new(Box::new(storage));
    let mut prompt = Prompt::new("Config test prompt content", None, None);
    prompts_api.add_prompt(&mut prompt).await?;

    let mut cmd = Command::cargo_bin("prompts-cli")?;
    cmd.arg("--config").arg(&config_path).arg("list");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains(&format!("{} - Config test prompt content", &prompt.hash[..12])));

    Ok(())
}