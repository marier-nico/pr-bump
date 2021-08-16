/// Tests to load configuration items.
///
/// An important note: it is very important to add the serial attribute macro to tests using env
/// vars, because otherwise the tests run into race conditions with each other. Also, the
/// `TestEnvVar` struct **must** be used and bound to a variable name. This binding will ensure the
/// env var is automatically removed at the end of the test. This cleanup is needed to also not
/// interfere with other tests.
use pr_bump_lib::{load_action_config, load_pr_bump_config};
use serial_test::serial;
use std::path::PathBuf;
use utils::{write_tmp_file, TestEnvVar};

mod utils;

#[test]
#[serial]
fn load_action_config_contains_all_info_when_available() {
    let _gh_repo = TestEnvVar::new("GITHUB_REPOSITORY", "some-owner/some-repo");
    let _gh_ws = TestEnvVar::new("GITHUB_WORKSPACE", "/path/to/workspace");
    let _gh_conf = TestEnvVar::new("INPUT_CONFIGURATION", "config_name.json");

    let result = load_action_config().unwrap();

    assert_eq!(result.repo.owner, "some-owner");
    assert_eq!(result.repo.repo, "some-repo");
    assert_eq!(result.workspace, PathBuf::from("/path/to/workspace"));
    assert_eq!(
        result.configuration_file,
        Some(PathBuf::from("/path/to/workspace/config_name.json"))
    );
}

#[test]
#[serial]
fn load_action_config_allows_not_setting_config_file() {
    let _gh_repo = TestEnvVar::new("GITHUB_REPOSITORY", "some-owner/some-repo");
    let _gh_ws = TestEnvVar::new("GITHUB_WORKSPACE", "/path/to/workspace");

    let result = load_action_config().unwrap();

    assert_eq!(result.configuration_file, None);
}

#[test]
#[serial]
fn load_action_config_returns_error_when_github_workspace_does_not_exist() {
    let _gh_repo = TestEnvVar::new("GITHUB_REPOSITORY", "some-owner/some-repo");

    let result = load_action_config();

    assert_eq!(result.is_err(), true);
}

#[test]
#[serial]
fn load_action_config_returns_error_when_github_repo_does_not_exist() {
    let _gh_ws = TestEnvVar::new("GITHUB_WORKSPACE", "/path/to/workspace");

    let result = load_action_config();

    assert_eq!(result.is_err(), true);
}

#[test]
#[serial]
fn load_action_config_returns_error_when_github_repo_cannot_be_parsed() {
    let _gh_repo = TestEnvVar::new("GITHUB_REPOSITORY", "some-owner|some-repo");
    let _gh_ws = TestEnvVar::new("GITHUB_WORKSPACE", "/path/to/workspace");

    let result = load_action_config();

    assert_eq!(result.is_err(), true);
}

#[test]
fn load_pr_bump_config_loads_full_config() {
    let config = r#"{
        "base_branches": [
            "main",
            "master"
        ],
        "bump_files": [
            "Cargo.toml",
            "package.json"
        ],
        "categories": [
            {
                "labels": ["bug", "docs"],
                "semver_part": "patch"
            },
            {
                "labels": ["custom_feature_label"],
                "semver_part": "minor"
            },
            {
                "labels": ["big_release"],
                "semver_part": "major"
            }
        ]
    }"#;
    let config_path = write_tmp_file(config);

    let loaded_config = load_pr_bump_config(&config_path.path());

    assert_eq!(loaded_config.is_ok(), true);
}

#[test]
fn load_pr_bump_config_loads_partial_config() {
    let config = r#"{
        "base_branches": [
            "main",
            "master"
        ]
    }"#;
    let config_path = write_tmp_file(config);

    let loaded_config = load_pr_bump_config(&config_path.path());

    assert_eq!(loaded_config.is_ok(), true);
    assert_eq!(loaded_config.as_ref().unwrap().bump_files.is_none(), true);
    assert_eq!(loaded_config.as_ref().unwrap().categories.is_none(), true);
}

#[test]
fn load_pr_bump_config_ignores_additional_unnecessary_values() {
    let config = r#"{
        "base_branches": [
            "main",
            "master"
        ],
        "some-key-that-should-be-ignored": 0
    }"#;
    let config_path = write_tmp_file(config);

    let loaded_config = load_pr_bump_config(&config_path.path());

    assert_eq!(loaded_config.is_ok(), true);
    assert_eq!(loaded_config.as_ref().unwrap().bump_files.is_none(), true);
    assert_eq!(loaded_config.as_ref().unwrap().categories.is_none(), true);
}
