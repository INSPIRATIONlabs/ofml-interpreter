//! Integration tests for CLI configure command (T017)

use std::process::Command;

/// Helper to run the ofml binary with arguments
fn run_ofml(args: &[&str]) -> (i32, String, String) {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()
        .expect("Failed to execute command");

    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    (exit_code, stdout, stderr)
}

#[test]
fn test_manufacturers_command_exists() {
    let (code, stdout, _stderr) = run_ofml(&["manufacturers", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("List available manufacturers"));
}

#[test]
fn test_articles_command_exists() {
    let (code, stdout, _stderr) = run_ofml(&["articles", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("List articles"));
}

#[test]
fn test_configure_command_exists() {
    let (code, stdout, _stderr) = run_ofml(&["configure", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("Configure a product"));
}

#[test]
fn test_configure_command_has_json_flag() {
    let (code, stdout, _stderr) = run_ofml(&["configure", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("--json"));
}

#[test]
fn test_configure_command_has_export_flag() {
    let (code, stdout, _stderr) = run_ofml(&["configure", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("--export"));
}

#[test]
fn test_configure_command_has_price_date_flag() {
    let (code, stdout, _stderr) = run_ofml(&["configure", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("--price-date"));
}

#[test]
fn test_configure_command_has_list_properties_flag() {
    let (code, stdout, _stderr) = run_ofml(&["configure", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("--list-properties"));
}

#[test]
fn test_tui_command_exists() {
    let (code, stdout, _stderr) = run_ofml(&["tui", "--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("Terminal UI"));
}

#[test]
fn test_version_flag() {
    let (code, stdout, _stderr) = run_ofml(&["--version"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("ofml"));
}

#[test]
fn test_help_flag() {
    let (code, stdout, _stderr) = run_ofml(&["--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("manufacturers"));
    assert!(stdout.contains("articles"));
    assert!(stdout.contains("configure"));
    assert!(stdout.contains("tui"));
}

#[test]
fn test_verbose_flag_exists() {
    let (code, stdout, _stderr) = run_ofml(&["--help"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("-v") || stdout.contains("--verbose"));
}

// Tests that require actual OFML data
#[cfg(feature = "integration_with_data")]
mod with_ofml_data {
    use super::*;

    const DATA_PATH: &str = "/workspace/ofmldata";

    #[test]
    fn test_manufacturers_lists_vitra() {
        let (code, stdout, _stderr) = run_ofml(&["manufacturers", DATA_PATH]);
        assert_eq!(code, 0);
        assert!(stdout.contains("vitra"));
    }

    #[test]
    fn test_manufacturers_json_output() {
        let (code, stdout, _stderr) = run_ofml(&["manufacturers", DATA_PATH, "--json"]);
        assert_eq!(code, 0);
        assert!(stdout.contains("\"manufacturers\""));
        assert!(stdout.starts_with("{"));
    }

    #[test]
    fn test_articles_for_vitra() {
        let (code, stdout, _stderr) = run_ofml(&["articles", DATA_PATH, "vitra"]);
        assert_eq!(code, 0);
        assert!(stdout.contains("Artikel"));
    }

    #[test]
    fn test_articles_invalid_manufacturer() {
        let (code, _stdout, stderr) = run_ofml(&["articles", DATA_PATH, "nonexistent"]);
        assert_ne!(code, 0);
        assert!(stderr.contains("nicht gefunden") || stderr.contains("not found"));
    }

    #[test]
    fn test_configure_list_properties() {
        let (code, stdout, _stderr) = run_ofml(&[
            "configure",
            DATA_PATH,
            "vitra",
            "OiBlock",
            "--list-properties",
        ]);
        assert_eq!(code, 0);
        assert!(stdout.contains("Properties"));
    }

    #[test]
    fn test_configure_json_output() {
        let (code, stdout, _stderr) = run_ofml(&[
            "configure",
            DATA_PATH,
            "vitra",
            "OiBlock",
            "--json",
        ]);
        assert_eq!(code, 0);
        assert!(stdout.contains("\"manufacturer_id\""));
        assert!(stdout.contains("\"article_id\""));
    }
}
