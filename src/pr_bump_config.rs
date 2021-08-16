use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
};

use eyre::Context;
use serde::Deserialize;

use crate::github::Branch;

type Label = String;
#[derive(Deserialize, Debug)]
pub enum SemverPart {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "major")]
    Major,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub labels: Vec<Label>,
    pub semver_part: SemverPart,
}

impl Category {
    pub fn new(labels: Vec<Label>, semver_part: SemverPart) -> Self {
        Category {
            labels,
            semver_part,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct PrBumpConfig {
    pub base_branches: Option<Vec<Branch>>,
    pub bump_files: Option<Vec<PathBuf>>,
    pub categories: Option<Vec<Category>>,
}

impl PrBumpConfig {
    pub fn merge(&mut self, other: Self) {
        if self.base_branches.is_none() {
            self.base_branches = other.base_branches;
        }

        if self.bump_files.is_none() {
            self.bump_files = other.bump_files
        }

        if self.categories.is_none() {
            self.categories = other.categories
        }
    }
}

type ConfigFile<'a> = &'a Path;
impl TryFrom<ConfigFile<'_>> for PrBumpConfig {
    type Error = eyre::Error;

    fn try_from(config: ConfigFile) -> Result<Self, Self::Error> {
        let mut config_content = fs::read_to_string(config).wrap_err(format!(
            "Could not read configuration at '{}'",
            config.to_string_lossy()
        ))?;

        config_content.retain(|c| !c.is_control());

        let deserialized_config = serde_json::from_str(&config_content)?;

        Ok(deserialized_config)
    }
}

impl Default for PrBumpConfig {
    fn default() -> Self {
        PrBumpConfig {
            base_branches: None,
            bump_files: Some(Vec::new()),
            categories: Some(vec![
                Category::new(
                    vec![
                        "bug".to_string(),
                        "docs".to_string(),
                        "documentation".to_string(),
                        "fix".to_string(),
                        "patch".to_string(),
                    ],
                    SemverPart::Patch,
                ),
                Category::new(
                    vec![
                        "feat".to_string(),
                        "feature".to_string(),
                        "minor".to_string(),
                    ],
                    SemverPart::Minor,
                ),
                Category::new(
                    vec!["breaking".to_string(), "major".to_string()],
                    SemverPart::Major,
                ),
            ]),
        }
    }
}
