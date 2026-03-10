use assert_cmd::Command;

fn cmd() -> Command {
    Command::cargo_bin("sam-env-diff").unwrap()
}

fn fixtures(f: &str) -> String {
    format!("fixtures/{}", f)
}

#[test]
fn no_args_exits_2() {
    cmd().assert().failure().code(2);
}

#[test]
fn missing_right_exits_2() {
    cmd().arg("fixtures/left.env").assert().failure().code(2);
}

#[test]
fn bad_file_exits_2() {
    cmd()
        .args(["nonexistent.env", "also_nonexistent.env"])
        .assert()
        .failure()
        .code(2);
}

#[test]
fn identical_files_exits_0() {
    cmd()
        .args([&fixtures("left.env"), &fixtures("left.env")])
        .assert()
        .success()
        .code(0);
}

#[test]
fn diff_files_exits_1() {
    cmd()
        .args([&fixtures("left.env"), &fixtures("right.env")])
        .assert()
        .failure()
        .code(1);
}

#[test]
fn bot_mode_valid_json() {
    let output = cmd()
        .args([&fixtures("left.env"), &fixtures("right.env"), "--bot"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .expect("bot mode output should be valid JSON");

    assert!(parsed["left"].is_string());
    assert!(parsed["right"].is_string());
    assert!(parsed["missing"].is_array());
    assert!(parsed["extra"].is_array());
    assert!(parsed["changed"].is_array());
    assert!(parsed["match"].is_number());
    assert!(parsed["ok"].is_boolean());
}

#[test]
fn bot_mode_no_ansi() {
    let output = cmd()
        .args([&fixtures("left.env"), &fixtures("right.env"), "--bot"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    // ANSI escape starts with ESC (0x1b)
    assert!(!stdout.contains('\x1b'), "bot mode should not contain ANSI codes");
}

#[test]
fn bot_help_valid_json() {
    let output = cmd()
        .arg("--bot-help")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout)
        .expect("--bot-help output should be valid JSON");

    assert_eq!(parsed["tool"], "sam-env-diff");
    assert!(parsed["flags"].is_object());
    assert!(parsed["exit_codes"].is_object());
}

#[test]
fn bot_help_exits_0() {
    cmd().arg("--bot-help").assert().success().code(0);
}

#[test]
fn output_to_file() {
    let out = "/tmp/sam-env-diff-test-output.json";
    cmd()
        .args([&fixtures("left.env"), &fixtures("right.env"), "-o", out])
        .assert()
        .failure()
        .code(1); // differences exist

    let content = std::fs::read_to_string(out).expect("output file should exist");
    let parsed: serde_json::Value = serde_json::from_str(&content)
        .expect("output file should be valid JSON");
    assert!(parsed["ok"].is_boolean());
    std::fs::remove_file(out).ok();
}

#[test]
fn values_masked_by_default() {
    let output = cmd()
        .args([&fixtures("left.env"), &fixtures("right.env"), "--bot"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // changed values should be masked
    for entry in parsed["changed"].as_array().unwrap() {
        let left = entry["left"].as_str().unwrap();
        assert!(left.starts_with("****"), "value should be masked: {}", left);
    }
}

#[test]
fn reveal_shows_full_values() {
    let output = cmd()
        .args([&fixtures("left.env"), &fixtures("right.env"), "--bot", "--reveal"])
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();

    // With --reveal, values should NOT start with ****
    for entry in parsed["changed"].as_array().unwrap() {
        let left = entry["left"].as_str().unwrap();
        assert!(!left.starts_with("****"), "value should not be masked with --reveal: {}", left);
    }
}

#[test]
fn all_flag_shows_match() {
    let output = cmd()
        .args([&fixtures("left.env"), &fixtures("right.env"), "--bot"])
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let match_count = parsed["match"].as_u64().unwrap();
    assert!(match_count > 0, "should have matching keys");
}
