use semver::Version;

use crate::github::PullRequest;

// TODO: Make these configurable (maybe use a hashmap or something)
fn label_to_bump_level(label: String) -> BumpLevel {
    match label.as_str() {
        "fix" => BumpLevel::Patch,
        "bug" => BumpLevel::Patch,
        "documentation" => BumpLevel::Patch,
        "feature" => BumpLevel::Minor,
        "enhancement" => BumpLevel::Minor,
        "breaking" => BumpLevel::Major,
        _ => BumpLevel::None,
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum BumpLevel {
    None,
    Patch,
    Minor,
    Major,
}

impl BumpLevel {
    fn from_pulls(pulls: impl Iterator<Item = PullRequest>) -> Self {
        pulls
            .flat_map(|pull| pull.labels.into_iter())
            .map(label_to_bump_level)
            .max()
            .unwrap_or(BumpLevel::None)
    }
}

pub fn bump_version(version: &Version, pulls: impl Iterator<Item = PullRequest>) -> Version {
    let mut next_version = version.clone();

    match BumpLevel::from_pulls(pulls) {
        BumpLevel::Major => {
            next_version.major += 1;
            next_version.minor = 0;
            next_version.patch = 0;
        }
        BumpLevel::Minor => {
            next_version.minor += 1;
            next_version.patch = 0;
        }
        BumpLevel::Patch => next_version.patch += 1,
        BumpLevel::None => {}
    }

    next_version
}
