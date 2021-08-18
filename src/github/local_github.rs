use super::GitHubOperations;
use crate::{PullRequest, Release};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::{eyre, Result};

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

    async fn get_pulls<'a, Branch>(
        &self,
        _base: Option<impl Iterator<Item = Branch> + 'async_trait>,
        merged_after: &DateTime<Utc>,
    ) -> Result<Self::PullIter>
    where
        Branch: AsRef<str>,
    {
        let merged_after = *merged_after; // Copy
        Ok(Box::new(
            self.pulls
                .clone()
                .into_iter()
                .filter(move |pr| pr.merged_at.unwrap() > merged_after),
        ))
    }

    async fn get_latest_release(&self) -> Result<Release> {
        self.releases
            .last()
            .cloned()
            .ok_or_else(|| eyre!("No releases"))
    }
}
