use crate::{timeframe::Timeframe, Result};
use chrono::TimeDelta; // TODO@ricab remove
use futures_util::TryStreamExt;
use octocrab::params::pulls::Sort;
use octocrab::params::Direction;
use tokio::pin;

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

        // TODO@ricab place default at clap level
        // TODO@ricab no magic numbers
        let days = 15;
        let delta = TimeDelta::days(days);
        let period = period.unwrap_or(
            Timeframe::from_delta(delta)
                .expect("A {days} days delta should result in a valid timeframe"),
        );

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

            if period.is_inside(ts) {
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
