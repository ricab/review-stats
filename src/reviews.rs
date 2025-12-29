use crate::{timeframe::Timeframe, repo_instance::RepoInstance, Result};
use futures_util::TryStreamExt;
use octocrab::params::pulls::Sort;
use octocrab::params::Direction;
use tokio::pin;

/// Statistics on GitHub reviews
#[derive(Debug)]
pub struct Stats;

// TODO@ricab test...

impl Stats {
    pub async fn build(repo: RepoInstance,
                       reviewers: Vec<String>,
                       period: Timeframe) -> Result<Self> {
        println!("Repository: {}", repo);
        println!("Users: {}", reviewers.join(" "));

        Stats::fetch_pulls(repo, period).await;
        Ok(Self {})
    }
}

// private helpers
impl Stats {
    async fn fetch_pulls(RepoInstance {owner, repo}: RepoInstance, period: Timeframe) {
        let octocrab = octocrab::instance();
        let repo_pulls = octocrab.pulls(owner, repo);
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
            // TODO@ricab why could this fail?
            let ts = pull.created_at.expect("Pull request should have a timestamp");

            if period.contains(ts) {
                // TODO@ricab log
                println!("Considering pull request #{} created at {ts}", pull.number);
                hit = true; // we've entered the period
                pulls.push(pull);
            } else if hit {
                break; // we're now looking past the period
            }
        }

        println!("Num pulls: {}", pulls.len())
    }
}
