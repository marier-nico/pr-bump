use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use eyre::Result;
use semver::Version;

#[async_trait(?Send)]
pub trait GitHubOperations {
    type PullIter: Iterator<Item = PullRequest>;

    async fn get_pulls<'a, Branch, Label>(
        &self,
        bases: Option<impl Iterator<Item = Branch> + 'async_trait>,
        ignore_labels: &[Label],
        merged_after: &DateTime<Utc>,
    ) -> Result<Self::PullIter>
    where
        Branch: AsRef<str>,
        Label: AsRef<str>;
    async fn get_latest_release(&self) -> Result<Release>;
}

#[derive(Debug, Clone)]
pub struct PullRequest {
    pub labels: Vec<String>,
    pub merged_at: Option<DateTime<Utc>>,
}

impl PullRequest {
    pub fn new(labels: Vec<String>, merged_at: Option<DateTime<Utc>>) -> Self {
        PullRequest { labels, merged_at }
    }
}

#[derive(Debug, Clone)]
pub struct Release {
    pub tag_name: String,
    pub created_at: DateTime<Utc>,
}

impl Release {
    pub fn new(tag_name: String, created_at: DateTime<Utc>) -> Self {
        Release {
            tag_name,
            created_at,
        }
    }

    /// Get the semver version for a GitHub release.
    ///
    /// It is assumed that the tag name associated with the release will be a valid semver version
    /// which may be preceded by a `v` prefix. For example, `v1.2.3` is valid. Semver prerelease and
    /// build metadata is also acceptable.
    pub fn get_version(&self) -> Result<Version> {
        let version = self
            .tag_name
            .strip_prefix('v')
            .unwrap_or_else(|| self.tag_name.as_str());

        Ok(Version::parse(version)?)
    }
}

impl Default for Release {
    fn default() -> Self {
        Release {
            tag_name: "0.1.0".to_string(),
            created_at: Utc.ymd(1970, 1, 1).and_hms(0, 0, 0),
        }
    }
}
