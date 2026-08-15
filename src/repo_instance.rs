use crate::{BoxedError, Result};

use regex::Regex;

use std::fmt::{self, Display, Formatter};
use std::sync::LazyLock;

#[derive(Debug, Clone)]
pub struct RepoInstance {
    pub owner: String,
    pub repo: String,
}

// Custom public interface
impl RepoInstance {
    pub fn new(owner: String, repo: String) -> Self {
        Self { owner, repo }
    }
}

impl Display for RepoInstance {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.owner, self.repo)
    }
}

impl std::str::FromStr for RepoInstance {
    type Err = BoxedError;
    fn from_str(s: &str) -> Result<Self> {
        let re = RepoInstance::ready_regex();

        let emsg = format!("Invalid repository format (expected '<owner>/<repo>', regex: {})",
                           re.as_str());
        let captures = re.captures(s).ok_or(emsg)?;

        Ok(RepoInstance::new(captures[1].to_string(), captures[2].to_string()))
    }
}

// private helpers
impl RepoInstance {
    fn ready_regex() -> &'static Regex {
        const OWNER_PATTERN: &str = r"[\w-]{1,39}";
        const REPO_PATTERN: &str = r"[\w\.-]{1,39}";
        static REGEX: LazyLock<Regex> = LazyLock::new(|| {
            let full_pattern = format!("^({OWNER_PATTERN})/({REPO_PATTERN})$");
            Regex::new(&full_pattern).expect("Pattern should be valid")
        });
        &REGEX
    }
}
