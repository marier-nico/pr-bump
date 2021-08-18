use eyre::{eyre, Context, Result};
use log::info;
use std::{env, path::PathBuf};

pub struct ActionConfig {
    pub repo: Repo,
    pub workspace: PathBuf,
    pub configuration_file: Option<PathBuf>,
    pub github_token: Option<String>,
}

pub struct Repo {
    pub owner: String,
    pub repo: String,
}

impl Repo {
    pub fn try_from_env() -> Result<Self> {
        info!("Reading value for GITHUB_REPOSITORY");
        let repo_info = env::var("GITHUB_REPOSITORY")
            .wrap_err("Could not read GITHUB_REPOSITORY to gather repository info")?;

        let mut parts = repo_info.split('/');
        Ok(Repo {
            owner: parts
                .next()
                .ok_or_else(|| eyre!("Repository owner info could not be parsed"))?
                .to_owned(),
            repo: parts
                .next()
                .ok_or_else(|| eyre!("Repository name could not be parsed"))?
                .to_owned(),
        })
    }
}

impl ActionConfig {
    pub fn try_from_env() -> Result<Self> {
        info!("Reading value for GITHUB_WORKSPACE");
        let workspace_path = env::var("GITHUB_WORKSPACE")
            .wrap_err("Could not read GITHUB_WORKSPACE to find the workspace path")?;
        let workspace_path = PathBuf::from(workspace_path);

        info!("Reading value for INPUT_CONFIGURATION");
        let configuration_file = env::var("INPUT_CONFIGURATION").ok();
        let configuration_file = match configuration_file {
            Some(value) => {
                if value.is_empty() {
                    None
                } else {
                    Some(workspace_path.join(value))
                }
            }
            None => None,
        };

        info!("Reading value for GITHUB_TOKEN");
        let github_token = env::var("GITHUB_TOKEN").ok();

        Ok(ActionConfig {
            repo: Repo::try_from_env()?,
            workspace: workspace_path,
            configuration_file,
            github_token,
        })
    }
}
