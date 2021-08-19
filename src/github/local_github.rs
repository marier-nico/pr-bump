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

    async fn get_pulls<'a, Branch, Label>(
        &self,
        _base: Option<impl Iterator<Item = Branch> + 'async_trait>,
        ignored_labels: &[Label],
        merged_after: &DateTime<Utc>,
    ) -> Result<Self::PullIter>
    where
        Branch: AsRef<str>,
        Label: AsRef<str>,
    {
        let merged_after = *merged_after; // Copy
        let ignored_labels: Vec<String> = ignored_labels
            .iter()
            .map(|l| l.as_ref().to_string())
            .collect();

        Ok(Box::new(
            self.pulls
                .clone()
                .into_iter()
                .filter(move |pr| pr.merged_at.unwrap() > merged_after)
                .filter(move |pr| {
                    pr.labels.iter().all(|label| {
                        !ignored_labels
                            .iter()
                            .any(|ignored_label| label == ignored_label)
                    })
                }),
        ))
    }

    async fn get_latest_release(&self) -> Result<Release> {
        self.releases
            .last()
            .cloned()
            .ok_or_else(|| eyre!("No releases"))
    }
}
