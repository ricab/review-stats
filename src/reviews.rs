use crate::{timeframe::Timeframe, Result};
use futures_util::TryStreamExt;
use octocrab::params::pulls::Sort;
use octocrab::params::Direction;
use tokio::pin;

/// Statistics on GitHub reviews
#[derive(Debug)]
pub struct Stats;

impl Stats {
    pub async fn build(repo: String, reviewers: Vec<String>, period: Timeframe) -> Result<Self> {
        println!("Repository: {}", repo);
        println!("Users: {}", reviewers.join(" "));

        Stats::fetch_pulls(period).await;
        Ok(Self {})
    }
}

// private helpers
impl Stats {
    async fn fetch_pulls(period: Timeframe) {
        let octocrab = octocrab::instance();
        let repo_pulls = octocrab.pulls("octocat", "hello-world"); // TODO@ricab parameterize repo
        let stream = repo_pulls
            .list()
            .sort(Sort::Created)
            .direction(Direction::Descending)
            .send()
            .await
            .unwrap()
            .into_stream(&octocrab); // TODO@ricab return result

        pin!(stream);
        let mut hit = false;
        let mut pulls = Vec::new();

        while let Some(pull) = stream.try_next().await.unwrap() {
            // TODO@ricab return result
            let ts = pull
                .created_at
                .expect("Pull request should have a timestamp"); // TODO@ricab why could this fail?

            if period.contains(ts) {
                println!("Considering pull request #{} created at {ts}", pull.number); // TODO@ricab log
                hit = true; // we've entered the period
                pulls.push(pull);
            } else if hit {
                break; // we're now looking past the period
            }
        }

        println!("Num pulls: {}", pulls.len())
    }
}
