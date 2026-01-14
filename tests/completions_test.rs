//! Integration tests for shell completions generation

use assert_cmd::Command;
use predicates::prelude::*;

fn decodeck() -> Command {
    Command::cargo_bin("decodeck").unwrap()
}

fn get_completions_output(shell: &str) -> String {
    let output = decodeck()
        .args(["completions", shell])
        .output()
        .expect("Failed to execute command");
    String::from_utf8(output.stdout).expect("Invalid UTF-8 output")
}

// =============================================================================
// User Story 1: Bash Completions (P1)
// =============================================================================

#[test]
fn test_bash_completions_generates_output() {
    let output = get_completions_output("bash");
    assert!(!output.is_empty(), "Bash completions should not be empty");
}

#[test]
fn test_bash_completions_contains_complete_command() {
    let output = get_completions_output("bash");
    assert!(
        output.contains("complete -F") || output.contains("complete -C"),
        "Bash completions should contain 'complete' command registration"
    );
}

#[test]
fn test_bash_completions_includes_decode_subcommand() {
    let output = get_completions_output("bash");
    assert!(
        output.contains("decode"),
        "Bash completions should reference 'decode' subcommand"
    );
}

// =============================================================================
// User Story 2: Zsh Completions (P2)
// =============================================================================

#[test]
fn test_zsh_completions_generates_output() {
    let output = get_completions_output("zsh");
    assert!(!output.is_empty(), "Zsh completions should not be empty");
}

#[test]
fn test_zsh_completions_contains_function_definition() {
    let output = get_completions_output("zsh");
    assert!(
        output.contains("_decodeck"),
        "Zsh completions should contain '_decodeck' function definition"
    );
}

#[test]
fn test_zsh_completions_includes_decode_subcommand() {
    let output = get_completions_output("zsh");
    assert!(
        output.contains("decode"),
        "Zsh completions should reference 'decode' subcommand"
    );
}

// =============================================================================
// User Story 3: Fish Completions (P3)
// =============================================================================

#[test]
fn test_fish_completions_generates_output() {
    let output = get_completions_output("fish");
    assert!(!output.is_empty(), "Fish completions should not be empty");
}

#[test]
fn test_fish_completions_contains_complete_command() {
    let output = get_completions_output("fish");
    assert!(
        output.contains("complete -c decodeck"),
        "Fish completions should contain 'complete -c decodeck' commands"
    );
}

#[test]
fn test_fish_completions_includes_decode_subcommand() {
    let output = get_completions_output("fish");
    assert!(
        output.contains("decode"),
        "Fish completions should reference 'decode' subcommand"
    );
}

// =============================================================================
// User Story 4: PowerShell Completions (P4)
// =============================================================================

#[test]
fn test_powershell_completions_generates_output() {
    let output = get_completions_output("powershell");
    assert!(
        !output.is_empty(),
        "PowerShell completions should not be empty"
    );
}

#[test]
fn test_powershell_completions_contains_register_command() {
    let output = get_completions_output("powershell");
    assert!(
        output.contains("Register-ArgumentCompleter"),
        "PowerShell completions should contain 'Register-ArgumentCompleter'"
    );
}

#[test]
fn test_powershell_completions_includes_decode_subcommand() {
    let output = get_completions_output("powershell");
    assert!(
        output.contains("decode"),
        "PowerShell completions should reference 'decode' subcommand"
    );
}

// =============================================================================
// Edge Cases & Error Handling
// =============================================================================

#[test]
fn test_completions_invalid_shell_error() {
    decodeck()
        .args(["completions", "invalid_shell"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn test_completions_help_shows_shells() {
    decodeck()
        .args(["completions", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("bash"))
        .stdout(predicate::str::contains("zsh"))
        .stdout(predicate::str::contains("fish"))
        .stdout(predicate::str::contains("powershell"));
}
