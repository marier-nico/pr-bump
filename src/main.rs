use std::fs;

use octocrab::params::{self, pulls};
use regex::Regex;
use semver::Version;

/// Bump the version in a given file
///
/// The prefix is what comes immediately before the version number and is not a regex.
/// For example, to bump `Cargo.toml`, `prefix` could be `version = \"`. This is just
/// to make sure only the correct thing is bumped and not another random version number.
fn bump_version(
    path: &str,
    prefix: &str,
    old_version: &Version,
    new_version: &Version,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::read_to_string(path)?;

    let re = Regex::new(&format!("{}{}", prefix, old_version)).unwrap();
    let replaced = re.replace(&file, format!("{}{}", prefix, new_version.to_string()));

    fs::write(path, replaced.as_ref())?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let github = octocrab::instance();
    let pulls = github
        .pulls("marier-nico", "venv-wrapper")
        .list()
        .state(params::State::Closed)
        .base("main")
        .sort(pulls::Sort::Created)
        .send()
        .await?;

    let latest_release = github
        .repos("marier-nico", "venv-wrapper")
        .releases()
        .get_latest()
        .await?;

    let current_version = latest_release
        .tag_name
        .strip_prefix('v')
        .unwrap_or_else(|| latest_release.tag_name.as_str());
    let current_version = Version::parse(current_version)?;
    let mut next_version = current_version.clone();

    let latest_release_created_on = latest_release.created_at;

    // We only look at pulls that were merged AFTER the latest release's creation
    let eligible_pulls = pulls
        .into_iter()
        .filter(|p| p.merged_at.is_some() && p.merged_at.unwrap() > latest_release_created_on);

    let mut is_fix = false;
    let mut is_docs = false;
    let mut is_feats = false;
    let mut is_breaking = false;
    for pull in eligible_pulls {
        let pull = pull.labels.unwrap_or_default();

        for label in pull.iter() {
            match label.name.as_str() {
                "fix" => is_fix = true,
                "bug" => is_fix = true,
                "documentation" => is_docs = true,
                "feature" => is_feats = true,
                "enhancement" => is_feats = true,
                "breaking" => is_breaking = true,
                _ => continue,
            }
        }
    }

    if is_breaking {
        next_version.major += 1;
        next_version.minor = 0;
        next_version.patch = 0;
    } else if is_feats {
        next_version.minor += 1;
        next_version.patch = 0;
    } else if is_fix || is_docs {
        next_version.patch += 1;
    };

    if next_version != current_version {
        println!("New version: {}", next_version);
        bump_version(
            "/home/nmarier/Documents/Software/Projects/venv-wrapper/Cargo.toml",
            "version = \"",
            &current_version,
            &next_version,
        )?;
    }

    Ok(())
}
