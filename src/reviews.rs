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

        Stats::fetch_pulls(repo, period).await?;
        Ok(Self {})
    }
}

// private helpers
impl Stats {
    async fn fetch_pulls(RepoInstance {owner, repo}: RepoInstance, period: Timeframe) -> Result<()> {
        let octocrab = octocrab::instance();
        let repo_pulls = octocrab.pulls(owner, repo);
        let stream = repo_pulls
            .list()
            .sort(Sort::Created)
            .direction(Direction::Descending)
            .send()
            .await?
            .into_stream(&octocrab);

        pin!(stream);
        let mut pulls = Vec::new();

        while let Some(pull) = stream.try_next().await? {
            let ts = pull.created_at.expect("Pull request should have a timestamp");

            // We'll consider any pull requests that weren't created after the interesting period.
            // Those are the ones that could have gotten reviews in that period.
            if ts > period.end() {
                break; // we're now looking past the interesting period
            }

            // TODO@ricab log
            println!("Considering pull request #{} created at {ts}", pull.number);
            pulls.push(pull);
        }

        println!("Num pulls: {}", pulls.len());
        Ok(())
    }
}
