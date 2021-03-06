use super::{error::GitHubError, GitHubOperations};
use crate::{PullRequest, Release};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::{eyre, Context, Result};
use log::info;
use octocrab::{params::State, Octocrab};
use std::sync::Arc;

#[derive(Debug)]
pub struct GitHub {
    owner: String,
    repo: String,
    octocrab: Arc<Octocrab>,
}

impl GitHub {
    pub fn new(owner: &str, repo: &str, token: Option<String>) -> Result<Self> {
        let mut octocrab_builder = Octocrab::builder();
        if let Some(token) = token {
            octocrab_builder = octocrab_builder.personal_token(token);
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

    async fn get_pulls<'a, Branch, Label>(
        &self,
        mut bases: Option<impl Iterator<Item = Branch> + 'async_trait>,
        ignored_labels: &[Label],
        merged_after: &DateTime<Utc>,
    ) -> Result<Self::PullIter>
    where
        Branch: AsRef<str>,
        Label: AsRef<str>,
    {
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
            .filter(|pr| {
                let ignored = match &pr.labels {
                    Some(pr_labels) => pr_labels.iter().all(|label| {
                        ignored_labels
                            .iter()
                            .any(|ignored_label| label.name == ignored_label.as_ref())
                    }),
                    None => true, // No labels on the PR means we can ignore it
                };

                if ignored {
                    info!("???????  #{} - {} (ignored)", pr.number, pr.title);
                } else {
                    info!(
                        "???? #{} - {} (merged {:?})",
                        pr.number, pr.title, pr.merged_at
                    );
                }

                !ignored
            })
            .filter(move |pr| pr.merged_at.is_some() && &pr.merged_at.unwrap() > merged_after)
            .filter(|pr| {
                let pr_base = pr.base.label.split(':').last().unwrap_or_else(|| {
                    panic!("Unexpected format for PR base: '{}'", pr.base.label)
                });

                bases.is_none() || bases.as_mut().unwrap().any(|base| base.as_ref() == pr_base)
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
        info!("Querying GitHub for the latest release");
        let latest_release = self
            .octocrab
            .repos(&self.owner, &self.repo)
            .releases()
            .get_latest()
            .await;

        let simplified = match latest_release {
            Ok(rel) => match rel.created_at {
                Some(created_at) => {
                    info!("Found latest release (tag: {})", rel.tag_name);

                    Release::new(rel.tag_name, created_at)
                }
                None => return Err(eyre!("The latest release seems to have no creation date")),
            },
            Err(e) => match GitHubError::from(e) {
                GitHubError::NotFound => Release::default(),
                GitHubError::Other(e) => return Err(e),
            },
        };

        Ok(simplified)
    }
}
