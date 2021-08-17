use std::collections::HashSet;

use semver::Version;

use crate::PullRequest;

type Label = String;
pub struct BumpRules {
    patch_bump_labels: HashSet<Label>,
    minor_bump_labels: HashSet<Label>,
    major_bump_labels: HashSet<Label>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum BumpLevel {
    Patch,
    Minor,
    Major,
}

impl BumpRules {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_patch_labels(&mut self, labels: Vec<String>) {
        for label in labels {
            self.patch_bump_labels.insert(label);
        }
    }

    pub fn add_minor_labels(&mut self, labels: Vec<String>) {
        for label in labels {
            self.minor_bump_labels.insert(label);
        }
    }

    pub fn add_major_labels(&mut self, labels: Vec<String>) {
        for label in labels {
            self.major_bump_labels.insert(label);
        }
    }

    fn label_into_level(&self, label: Label) -> Option<BumpLevel> {
        if self.patch_bump_labels.contains(&label) {
            Some(BumpLevel::Patch)
        } else if self.minor_bump_labels.contains(&label) {
            Some(BumpLevel::Minor)
        } else if self.major_bump_labels.contains(&label) {
            Some(BumpLevel::Major)
        } else {
            None
        }
    }
}

impl Default for BumpRules {
    fn default() -> Self {
        BumpRules {
            patch_bump_labels: HashSet::new(),
            minor_bump_labels: HashSet::new(),
            major_bump_labels: HashSet::new(),
        }
    }
}

pub fn bump_version(
    current_version: &Version,
    rules: &BumpRules,
    pulls: impl Iterator<Item = PullRequest>,
) -> Version {
    let mut next_version = current_version.clone();

    let bump_level = pulls
        .flat_map(|pr| pr.labels.into_iter())
        .flat_map(|label| rules.label_into_level(label))
        .max();

    if let Some(level) = bump_level {
        match level {
            BumpLevel::Patch => {
                next_version.patch += 1;
            }
            BumpLevel::Minor => {
                next_version.patch = 0;
                next_version.minor += 1;
            }
            BumpLevel::Major => {
                next_version.patch = 0;
                next_version.minor = 0;
                next_version.major += 1;
            }
        }
    }

    next_version
}
