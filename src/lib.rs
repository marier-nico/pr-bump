use std::path::Path;

pub use bump_version::BumpRules;
use bump_version::{bump_in_file, bump_version};
use chrono::{DateTime, Utc};
use eyre::{Context, Result};
use github::GitHubOperations;
pub use github::{GitHub, LocalGitHub, PullRequest, Release};
use semver::Version;

mod bump_version;
mod github;

pub async fn get_latest_release(github: &GitHub) -> Result<Release>
where
    GitHub: GitHubOperations,
{
    github
        .get_latest_release()
        .await
        .wrap_err("Could not find latest release in GitHub")
}

pub async fn get_pulls<GitHub, Branch, PRs>(
    github: &GitHub,
    bases: Option<&Vec<Branch>>,
    merged_after: &DateTime<Utc>,
) -> Result<PRs>
where
    GitHub: GitHubOperations<PullIter = PRs>,
    Branch: AsRef<str> + Clone,
    PRs: Iterator<Item = PullRequest>,
{
    github
        .get_pulls(bases.map(|bases| bases.clone().into_iter()), merged_after)
        .await
        .wrap_err("Could not list pull requests in GitHub")
}

pub fn get_next_version(
    current_version: &Version,
    bump_rules: &BumpRules,
    pulls: impl Iterator<Item = PullRequest>,
) -> Version {
    bump_version(current_version, bump_rules, pulls)
}

// Bump the version in a given file
//
// The prefix is what comes immediately before the version number and is not a regex.
// For example, to bump `Cargo.toml`, `prefix` could be `version = \"`. This is just
// to make sure only the correct thing is bumped and not another random version number.
pub fn update_file(
    current_version: &Version,
    next_version: &Version,
    version_prefix: &str,
    file_path: &Path,
) -> Result<()> {
    bump_in_file(current_version, next_version, version_prefix, file_path)
}
