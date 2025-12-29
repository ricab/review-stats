use std::fmt::{Display, Formatter};
use crate::{BoxedError, Result};

pub struct RepoInstance {
    owner: String,
    repo: String,
}

// Custom public interface
impl RepoInstance {
    pub fn new(owner: String, repo: String) -> Self {
        Self { owner, repo }
    }
}

impl Display for RepoInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

impl std::str::FromStr for RepoInstance {
    type Err = BoxedError;
    fn from_str(s: &str) -> Result<Self> {
        match s.split_once('/') { // TODO@ricab prevent funny URL fragment injection
            Some((owner, repo)) if !owner.is_empty() && !repo.is_empty() => {
                Ok(Self::new(owner.to_string(), repo.to_string()))
            }
            _ => Err(format!("Invalid repository format '{}': expected 'owner/repo'", s).into()),
        }
    }
}
