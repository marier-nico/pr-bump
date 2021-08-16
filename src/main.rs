use eyre::Result;
use pr_bump_lib::get_next_version;
use pr_bump_lib::load_action_config;
use pr_bump_lib::load_pr_bump_config;
use pr_bump_lib::GitHub;
use pr_bump_lib::PrBumpConfig;

// TODO:
// - Setup configuration (with serde_json and a config struct)
// - Add docs to relevant public functions
// - Make sure the repo defines a valid action (https://docs.github.com/en/actions/creating-actions/creating-a-docker-container-action)
//   - Create the config file, the inputs, etc.
// - Make the README up to stuff, list all the inputs and outputs
//   - Give examples of how to integrate with the changelog builder (https://github.com/marketplace/actions/release-changelog-builder)
//   - Give examples of how to use stand-alone
// - (For the function to update the version in files), use any `Write` instead of a path to allow in-memory testing, or maybe just take a string slice and return a modified String

#[tokio::main]
async fn main() -> Result<()> {
    let action_config = load_action_config()?;
    let pr_bump_config = {
        if let Some(config) = action_config.configuration_file {
            let file_config = action_config.workspace.join(config);
            load_pr_bump_config(&file_config)?.merge(PrBumpConfig::default())
        } else {
            PrBumpConfig::default()
        }
    };

    let github = GitHub::new(
        &action_config.repo.owner,
        &action_config.repo.repo,
        action_config.github_token,
    )?;

    let next_version = get_next_version(github, pr_bump_config).await?;

    println!("Next version: {}", next_version);
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
