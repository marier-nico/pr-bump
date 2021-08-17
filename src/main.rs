use std::convert::TryFrom;

use eyre::Result;
use pr_bump_lib::{get_latest_release, get_next_version, get_pulls, GitHub};

use crate::{actions_config::ActionConfig, pr_bump_config::PrBumpConfig};
mod actions_config;
mod pr_bump_config;

// TODO:
// - Add docs to relevant public functions
// - Make sure the repo defines a valid action (https://docs.github.com/en/actions/creating-actions/creating-a-docker-container-action)
//   - Create the config file, the inputs, etc.
// - Make the README up to stuff, list all the inputs and outputs
//   - Give examples of how to integrate with the changelog builder (https://github.com/marketplace/actions/release-changelog-builder)
//   - Give examples of how to use stand-alone
// - (For the function to update the version in files), use any `Write` instead of a path to allow in-memory testing, or maybe just take a string slice and return a modified String
// - hook up action outputs with those defined in `action.yml`

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let action_config = ActionConfig::try_from_env()?;
    let pr_bump_config = {
        if let Some(config) = action_config.configuration_file {
            let file_config = action_config.workspace.join(config);
            PrBumpConfig::try_from(file_config.as_ref())?.merge(PrBumpConfig::default())
        } else {
            PrBumpConfig::default()
        }
    };

    let github = GitHub::new(
        &action_config.repo.owner,
        &action_config.repo.repo,
        action_config.github_token,
    )?;

    let latest = get_latest_release(&github).await?;
    let pulls = get_pulls(
        &github,
        pr_bump_config.base_branches.as_ref(),
        &latest.created_at,
    )
    .await?;

    let next_version = get_next_version(
        &latest.get_version()?,
        &pr_bump_config.get_bump_rules(),
        pulls,
    );

    println!("Next version: {}", next_version);
    // TODO: Bump in files

    /*if latest_releasse.get_version()? != next_version {
        println!("New version: {}", next_version);
        bump_version_in_file(
            "/home/nmarier/Documents/Software/Projects/venv-wrapper/Cargo.toml",
            "version = \"",
            &latest_releasse.get_version()?,
            &next_version,
        )?;
    }*/
    
    Ok(())
}
