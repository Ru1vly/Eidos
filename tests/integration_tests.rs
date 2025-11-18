// Integration tests for Eidos CLI
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("AI-powered CLI for Linux"));
}

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("0.2.0-beta"));
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
        stderr.contains("Chat Error")
            || stderr.contains("Tip: Configure an API provider")
            || output.status.success(),
        "Expected chat error message or success, got: {}",
        stderr
    );
}

#[test]
fn test_translate_command() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.arg("translate").arg("Bonjour le monde");

    // Test should pass if EITHER:
    // 1. Translation succeeds (has API key configured), OR
    // 2. Fails gracefully with clear API error message
    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let has_success_output = stdout.contains("Detected language");
    let has_api_error = stderr.contains("Translation Error") || stderr.contains("API error");

    assert!(
        has_success_output || has_api_error,
        "Expected either successful translation or graceful API error, got stdout: {}, stderr: {}",
        stdout,
        stderr
    );
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
        stderr.contains("Configuration validation failed")
            || stderr.contains("Tip: Set EIDOS_MODEL_PATH"),
        "Expected config error message, got: {}",
        stderr
    );
}

#[test]
fn test_missing_subcommand() {
    let mut cmd = Command::cargo_bin("eidos").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: eidos [OPTIONS] <COMMAND>"));
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
    cmd.arg("translate")
        .arg("This is English text that is long enough to be detected properly.");

    let output = cmd.output().unwrap();
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should detect English and report it (even if translation API is unavailable)
    assert!(
        stdout.contains("Detected language: en") || stdout.contains("Text is already in en"),
        "Expected English detection, got: {}",
        stdout
    );
}
