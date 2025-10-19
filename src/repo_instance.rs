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

impl std::str::FromStr for RepoInstance {
    type Err = BoxedError;
    fn from_str(s: &str) -> Result<Self> {
        todo!()
    }
}
