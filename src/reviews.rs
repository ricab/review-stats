use std::collections::HashMap;
use crate::{timeframe::Timeframe, repo_instance::RepoInstance, Result};
use futures_util::{TryStream, TryStreamExt};
use octocrab::models::pulls::PullRequest;
use octocrab::Octocrab;
use octocrab::params::pulls::Sort;
use octocrab::params::Direction;
use octocrab::params::State::All;
use octocrab::pulls::PullRequestHandler;
use tokio::pin;

/// Statistics on GitHub reviews
#[derive(Debug)]
pub struct Stats
{
    pub review_counts: HashMap<String, u32>
}

// TODO@ricab test...

impl Stats {
    pub async fn build(RepoInstance {owner, repo}: RepoInstance,
                       reviewers: Vec<String>,
                       period: Timeframe) -> Result<Self> {
        log::debug!("Repository: {}", repo);
        log::debug!("Users: {}", reviewers.join(" "));

        let octocrab = build_octocrab()?;
        let pull_handler = octocrab.pulls(owner, repo);
        let stream = pull_handler
            .list()
            .state(All)
            .sort(Sort::Updated)
            .direction(Direction::Descending)
            .send()
            .await?
            .into_stream(&octocrab);

        pin!(stream);

        let pulls = Stats::candidate_pulls(stream, &period).await?;
        let review_counts = Stats::count_reviews(&octocrab, pull_handler, pulls, reviewers, &period).await?;

        Ok(Self { review_counts })
    }
}

// private helpers
impl Stats {
    async fn candidate_pulls<S: TryStream<Ok=PullRequest, Error=octocrab::Error> + Unpin>(
        mut pull_stream: S, period: &Timeframe) -> Result<Vec<PullRequest>> {
        let mut pulls = Vec::new();

        while let Some(pull) = pull_stream.try_next().await? {
            let ts = pull.updated_at.expect("Pull request should have an update timestamp");

            if ts < period.begin() {
                break; // this is too old - we're now past PRs updated within the period
            }

            log::debug!("Considering pull request #{} as candidate (updated at {ts})", pull.number);
            pulls.push(pull);
        }

        log::debug!("Looking for reviews on {} pull requests", pulls.len());
        Ok(pulls)
    }

    async fn count_reviews(
        octocrab: &Octocrab,
        pull_handler: PullRequestHandler<'_>,
        pulls: Vec<PullRequest>,
        reviewers: Vec<String>,
        period: &Timeframe,
    ) -> Result<HashMap<String, u32>> {
        let mut counts: HashMap<String, u32> = HashMap::new();
        for pull in pulls {
            let reviews = pull_handler.list_reviews(pull.number).send().await?.into_stream(&octocrab);
            pin!(reviews);
            while let Some(review) = reviews.try_next().await? {
                let review_ts = review.submitted_at.expect("Pull request should have a timestamp");
                if period.contains(review_ts) {
                    let reviewer = review.user.expect("A review should have been made by someone").login;
                    if reviewers.is_empty() || reviewers.contains(&reviewer) {
                        *counts.entry(reviewer).or_insert(0) += 1;
                    }
                }
            }
        }

        Ok(counts)
    }
}

fn build_octocrab() -> Result<Octocrab> {
    let mut builder = Octocrab::builder();

    let token = std::env::var("GITHUB_TOKEN").unwrap_or_default();
    if !token.is_empty() {
        log::debug!("Authenticating with personal token from $GITHUB_TOKEN");
        builder = builder.personal_token(token);
    }

    Ok(builder.build()?)
}
