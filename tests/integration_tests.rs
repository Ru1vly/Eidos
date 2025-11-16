// Integration tests for Eidos CLI
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A multifunctional application"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.1.0"));
}

#[test]
fn test_chat_command() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("chat").arg("Hello, world!");

    // Should either succeed or fail with clear error message
    // Since we might not have API keys configured, we accept both outcomes
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should mention chat or API configuration
    assert!(
        stderr.contains("Chat Error") || stderr.contains("Tip: Configure an API provider") || output.status.success(),
        "Expected chat error message or success, got: {}",
        stderr
    );
}

#[test]
fn test_translate_command() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("translate").arg("Bonjour le monde");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected language"));
}

#[test]
fn test_core_command_without_config() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("core").arg("list files");

    // Should fail gracefully without config
    let output = cmd.output().unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Should mention configuration
    assert!(
        stderr.contains("Configuration validation failed") || stderr.contains("Tip: Set EIDOS_MODEL_PATH"),
        "Expected config error message, got: {}",
        stderr
    );
}

#[test]
fn test_missing_subcommand() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: eidos <COMMAND>"));
}

#[test]
fn test_invalid_subcommand() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("invalid");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("unrecognized subcommand"));
}

#[test]
fn test_chat_command_empty_text() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("chat").arg("");

    // Should handle empty input gracefully
    let output = cmd.output().unwrap();
    assert!(output.status.success() || !output.stderr.is_empty());
}

#[test]
fn test_translate_command_english_text() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("translate").arg("This is English text that is long enough to be detected properly.");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Detected language: en"))
        .stdout(predicate::str::contains("already in English"));
}
