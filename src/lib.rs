use std::{convert::TryFrom, fs, path::Path};

use actions_config::ActionConfig;
use bump_version::bump_version;
use eyre::Result;
use github::GitHubOperations;
pub use github::{GitHub, LocalGitHub, PullRequest, Release};
pub use pr_bump_config::PrBumpConfig;
use regex::Regex;
use semver::Version;

mod actions_config;
mod bump_version;
mod github;
mod pr_bump_config;

pub fn load_action_config() -> Result<ActionConfig> {
    ActionConfig::try_from_env()
}

pub fn load_pr_bump_config(config_path: &Path) -> Result<PrBumpConfig> {
    PrBumpConfig::try_from(config_path)
}

pub async fn get_next_version(github: impl GitHubOperations, config: PrBumpConfig) -> Result<Version> {
    let latest_releasse = github.get_latest_release().await?;

    let eligible_pulls = github
        .get_pulls_after(config.base_branches, latest_releasse.clone())
        .await?;
    let next_version = bump_version(&latest_releasse.get_version()?, eligible_pulls);

    Ok(next_version)
}

/// Bump the version in a given file
///
/// The prefix is what comes immediately before the version number and is not a regex.
/// For example, to bump `Cargo.toml`, `prefix` could be `version = \"`. This is just
/// to make sure only the correct thing is bumped and not another random version number.
fn _bump_version_in_file(
    path: &str,
    prefix: &str,
    old_version: &Version,
    new_version: &Version,
) -> Result<()> {
    let file = fs::read_to_string(path)?;

    let re = Regex::new(&format!("{}{}", prefix, old_version)).unwrap();
    let replaced = re.replace(&file, format!("{}{}", prefix, new_version.to_string()));

    fs::write(path, replaced.as_ref())?;

    Ok(())
}
