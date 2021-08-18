use super::{error::GitHubError, GitHubOperations};
use crate::{PullRequest, Release};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::{Context, Result};
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

    async fn get_pulls<'a, Branch>(
        &self,
        mut bases: Option<impl Iterator<Item = Branch> + 'async_trait>,
        merged_after: &DateTime<Utc>,
    ) -> Result<Self::PullIter>
    where
        Branch: AsRef<str>,
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
