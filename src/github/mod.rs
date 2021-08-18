pub use github_operations::{GitHubOperations, PullRequest, Release};
pub use local_github::LocalGitHub;
pub use real_github::GitHub;

mod error;
mod github_operations;
mod local_github;
mod real_github;
