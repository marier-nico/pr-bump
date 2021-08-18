use actions_tools::{close_group, group_lines};
use config::{actions_config::ActionConfig, pr_bump_config::PrBumpConfig};
use eyre::Result;
use log::{error, info, LevelFilter};
use pr_bump_lib::{get_latest_release, get_next_version, get_pulls, update_file, GitHub};
use std::convert::TryFrom;
use std::io::Write;

use crate::actions_tools::set_output;

mod actions_tools;
mod config;

fn setup_logging() {
    env_logger::Builder::from_default_env()
        .format(|f, record| match record.level() {
            log::Level::Error => writeln!(f, "::error::{}", record.args()),
            log::Level::Warn => writeln!(f, "::warning::{}", record.args()),
            log::Level::Info => writeln!(f, "{}", record.args()),
            log::Level::Debug => writeln!(f, "::debug::{}", record.args()),
            log::Level::Trace => writeln!(f, "::debug::{}", record.args()),
        })
        .filter(None, LevelFilter::Trace)
        .init();
}

async fn run_action() -> Result<()> {
    group_lines("⚙️  Reading input configuration");
    let action_config = ActionConfig::try_from_env()?;

    let pr_bump_config = {
        if let Some(config) = action_config.configuration_file {
            let file_config = action_config.workspace.join(config);
            PrBumpConfig::try_from(file_config.as_ref())?.merge(PrBumpConfig::default())
        } else {
            PrBumpConfig::default()
        }
    };
    close_group();

    let github = GitHub::new(
        &action_config.repo.owner,
        &action_config.repo.repo,
        action_config.github_token,
    )?;

    group_lines("🛳️  Finding latest release");
    let latest = get_latest_release(&github).await?;
    close_group();

    group_lines("📜  Reading pull requests");
    let pulls = get_pulls(
        &github,
        pr_bump_config.base_branches.as_ref(),
        &latest.created_at,
    )
    .await?;
    close_group();

    group_lines("🎯  Calculating version bump");
    let next_version = get_next_version(
        &latest.get_version()?,
        &pr_bump_config.get_bump_rules(),
        pulls,
    );
    close_group();

    group_lines("✏️  Updating files with the new version");
    for bump_file in &pr_bump_config.bump_files.unwrap() {
        let full_path = &action_config.workspace.join(&bump_file.path);

        update_file(
            &latest.get_version()?,
            &next_version,
            &bump_file.prefix,
            &full_path,
        )?;
    }
    close_group();

    if latest.get_version().unwrap() == next_version {
        info!(
            "✅ Done! Version did not change (current: {})",
            &next_version
        );
        set_output("has_bump", "false");
    } else {
        info!(
            "✅ Done! Performed a version bump: {} ➡ {}",
            &latest.get_version().unwrap(),
            &next_version
        );
        set_output("has_bump", "true");
    }

    set_output(
        "previous_version",
        &latest.get_version().unwrap().to_string(),
    );
    set_output("next_version", &next_version.to_string());

    Ok(())
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    setup_logging();

    let result = run_action().await;
    match result {
        Ok(_) => {}
        Err(e) => error!("💥  {}", e),
    }
}
