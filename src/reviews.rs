use crate::{timeframe::Timeframe, Result};
use octocrab::params::pulls::Sort;
use octocrab::params::Direction;

/// Statistics on GitHub reviews
#[derive(Debug)]
pub struct Stats;

impl Stats {
    pub async fn build(
        repo: String,
        reviewers: Option<Vec<String>>,
        period: Option<Timeframe>,
    ) -> Result<Self> {
        println!("Repository: {}", repo);

        if let Some(reviewers) = reviewers {
            println!("Users: {}", reviewers.join(" "));
        }

        if let Some(period) = period {
            println!("Period: {period}");
        }

        Stats::fetch_pulls().await;
        Ok(Self {})
    }
}

// private helpers
impl Stats {
    async fn fetch_pulls() {
        let octocrab = octocrab::instance();
        let repo_pulls = octocrab.pulls("octocat", "hello-world"); // TODO@ricab parameterize repo
        let page = repo_pulls
            .list()
            .sort(Sort::Created)
            .per_page(5)
            .page(1u32)
            .direction(Direction::Descending)
            .send()
            .await
            .unwrap(); // TODO@ricab return result
        let pulls = page.items;
        println!("{pulls:#?}");
        println!("Num pulls: {}", pulls.len())
    }
}
