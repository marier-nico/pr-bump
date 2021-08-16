use std::sync::Arc;

use async_trait::async_trait;
use chrono::{DateTime, TimeZone, Utc};
use eyre::{eyre, Context, Result};
use octocrab::{params::State, Octocrab};
use semver::Version;

#[derive(Debug)]
enum GitHubError {
    NotFound,
    Other(eyre::Error),
}

impl From<octocrab::Error> for GitHubError {
    fn from(value: octocrab::Error) -> Self {
        if let octocrab::Error::GitHub {
            source,
            backtrace: _,
        } = value
        {
            match source.message.as_str() {
                "Not Found" => return GitHubError::NotFound,
                _ => return GitHubError::Other(eyre!(source.message)),
            }
        }

        GitHubError::Other(eyre!(value.to_string()))
    }
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

pub type Branch = String;
#[async_trait(?Send)]
pub trait GitHubOperations {
    type PullIter: Iterator<Item = PullRequest>;

    async fn get_pulls_after(
        &self,
        bases: Option<Vec<Branch>>,
        release: Release,
    ) -> Result<Self::PullIter>;
    async fn get_latest_release(&self) -> Result<Release>;
}

#[derive(Debug)]
pub struct GitHub {
    owner: String,
    repo: String,
    octocrab: Arc<Octocrab>,
}

impl GitHub {
    pub fn new(owner: &str, repo: &str, token: Option<&str>) -> Result<Self> {
        let mut octocrab_builder = Octocrab::builder();
        if let Some(token) = token {
            octocrab_builder = octocrab_builder.personal_token(token.to_string());
        }

        octocrab::initialise(octocrab_builder).wrap_err("Could not initialize GitHub SDK")?;

        Ok(GitHub {
            owner: owner.to_string(),
            repo: repo.to_string(),
            octocrab: octocrab::instance(),
        })
    }
}

#[async_trait(?Send)]
impl GitHubOperations for GitHub {
    type PullIter = Box<dyn Iterator<Item = PullRequest>>;

    async fn get_pulls_after(
        &self,
        bases: Option<Vec<Branch>>,
        release: Release,
    ) -> Result<Self::PullIter> {
        let pulls = self
            .octocrab
            .pulls(&self.owner, &self.repo)
            .list()
            .state(State::Closed)
            .per_page(100)
            .send()
            .await?;

        let eligible = pulls
            .into_iter()
            .filter(move |pr| pr.merged_at.is_some() && pr.merged_at.unwrap() > release.created_at)
            .filter(|pr| {
                let pr_base = pr.base.label.split(':').last().unwrap_or_else(|| {
                    panic!("Unexpected format for PR base: '{}'", pr.base.label)
                });

                println!(
                    "#{} [{:?}] {} -> {}",
                    pr.number, pr.merged_at, pr.title, pr_base
                );
                bases.is_none() || bases.as_ref().unwrap().contains(&pr_base.to_owned())
            });

        let simplified: Vec<PullRequest> = eligible
            .into_iter()
            .map(|p| {
                let labels = p
                    .labels
                    .unwrap_or_default()
                    .iter()
                    .map(|l| l.name.clone())
                    .collect();

                PullRequest::new(labels, p.merged_at)
            })
            .collect(); // We collect to avoid cloning `bases` for every PR

        Ok(Box::new(simplified.into_iter()))
    }

    async fn get_latest_release(&self) -> Result<Release> {
        let latest_release = self
            .octocrab
            .repos(&self.owner, &self.repo)
            .releases()
            .get_latest()
            .await;

        let simplified = match latest_release {
            Ok(rel) => Release::new(rel.tag_name, rel.created_at),
            Err(e) => match GitHubError::from(e) {
                GitHubError::NotFound => Release::default(),
                GitHubError::Other(e) => return Err(e),
            },
        };

        Ok(simplified)
    }
}

#[derive(Debug)]
pub struct LocalGitHub {
    pulls: Vec<PullRequest>,
    releases: Vec<Release>,
}

impl LocalGitHub {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_pull(&mut self, pull: PullRequest) {
        self.pulls.push(pull);
    }

    pub fn add_release(&mut self, release: Release) {
        self.releases.push(release);
    }
}

impl Default for LocalGitHub {
    fn default() -> Self {
        LocalGitHub {
            pulls: Vec::new(),
            releases: Vec::new(),
        }
    }
}

#[async_trait(?Send)]
impl GitHubOperations for LocalGitHub {
    type PullIter = Box<dyn Iterator<Item = PullRequest>>;

    async fn get_pulls_after(
        &self,
        _base: Option<Vec<Branch>>,
        release: Release,
    ) -> Result<Self::PullIter> {
        Ok(Box::new(self.pulls.clone().into_iter().filter(move |pr| {
            pr.merged_at.unwrap() > release.created_at
        })))
    }

    async fn get_latest_release(&self) -> Result<Release> {
        self.releases
            .last()
            .cloned()
            .ok_or_else(|| eyre!("No releases"))
    }
}
