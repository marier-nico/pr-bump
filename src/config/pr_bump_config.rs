use std::{
    convert::TryFrom,
    fs,
    path::{Path, PathBuf},
};

use eyre::Context;
use log::info;
use pr_bump_lib::BumpRules;
use serde::Deserialize;

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
pub struct BumpFile {
    pub path: PathBuf,

    #[serde(default)]
    pub prefix: String,
}

#[derive(Deserialize, Debug)]
pub struct PrBumpConfig {
    pub base_branches: Option<Vec<String>>,
    pub bump_files: Option<Vec<BumpFile>>,
    pub categories: Option<Vec<Category>>,
}

impl PrBumpConfig {
    pub fn merge(mut self, other: Self) -> Self {
        if self.base_branches.is_none() {
            self.base_branches = other.base_branches;
        }

        if self.bump_files.is_none() {
            self.bump_files = other.bump_files
        }

        if self.categories.is_none() {
            self.categories = other.categories
        }

        self
    }

    pub fn get_bump_rules(&self) -> BumpRules {
        let mut rules = BumpRules::new();

        if let Some(categories) = self.categories.as_ref() {
            for category in categories {
                match category.semver_part {
                    SemverPart::Patch => rules.add_patch_labels(category.labels.clone()),
                    SemverPart::Minor => rules.add_minor_labels(category.labels.clone()),
                    SemverPart::Major => rules.add_major_labels(category.labels.clone()),
                }
            }
        }

        rules
    }
}

type ConfigFile<'a> = &'a Path;
impl TryFrom<ConfigFile<'_>> for PrBumpConfig {
    type Error = eyre::Error;

    fn try_from(config: ConfigFile) -> Result<Self, Self::Error> {
        info!("Trying to read config file '{}'", config.to_string_lossy());
        let mut config_content = fs::read_to_string(config).wrap_err(format!(
            "Could not read configuration at '{}'",
            config.to_string_lossy()
        ))?;

        config_content.retain(|c| !c.is_control());

        info!("Parsing configuration file");
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
                        "enhancement".to_string(),
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
