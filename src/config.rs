use serde::Deserialize;

use crate::github::Branch;

type Label = String;
#[derive(Deserialize)]
pub enum SemverPart {
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "minor")]
    Minor,
    #[serde(rename = "major")]
    Major,
}

#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct Config {
    categories: Vec<Category>,
    base_branches: Option<Vec<Branch>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            categories: vec![
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
            ],
            base_branches: None,
        }
    }
}
