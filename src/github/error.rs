use eyre::eyre;

#[derive(Debug)]
pub enum GitHubError {
    NotFound,
    Other(eyre::Error),
}

impl From<octocrab::Error> for GitHubError {
    fn from(value: octocrab::Error) -> Self {
        if let octocrab::Error::GitHub {
            source,
            backtrace: _,
        } = value
        {
            match source.message.as_str() {
                "Not Found" => return GitHubError::NotFound,
                _ => return GitHubError::Other(eyre!(source.message)),
            }
        }

        GitHubError::Other(eyre!(value.to_string()))
    }
}
