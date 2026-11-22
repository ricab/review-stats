use std::cmp::Reverse;
use std::collections::HashMap;
use std::env::VarError;
use crate::{timeframe::Timeframe, repo_instance::RepoInstance, Result};
use futures_util::{TryStream, TryStreamExt};
use octocrab::models::pulls::{Comment, PullRequest, Review, ReviewState};
use octocrab::models::ReviewId;
use octocrab::Octocrab;
use octocrab::params::pulls::Sort;
use octocrab::params::Direction;
use octocrab::params::State::All;
use octocrab::pulls::PullRequestHandler;
use tokio::pin;

/// A submitted GitHub PR review
#[derive(Debug)]
pub struct Record {
    pub url: String,
}

impl Record {
    fn from_github(review: &Review) -> Self {
        Self { url: review.html_url.to_string() }
    }
}

/// A record of interesting reviews, grouped by reviewer
#[derive(Debug)]
pub struct Ledger
{
    pub reviews: HashMap<String, Vec<Record>>
}

// TODO test...

impl Ledger {
    pub async fn build(RepoInstance {owner, repo}: RepoInstance,
                       reviewers: Vec<String>,
                       period: Timeframe) -> Result<Self> {
        log::debug!("Repository: {}", repo);
        log::debug!("Users: {}", reviewers.join(" "));

        let octocrab = build_octocrab()?;
        let pull_handler = octocrab.pulls(owner.clone(), repo.clone());
        let stream = pull_handler
            .list()
            .state(All)
            .sort(Sort::Updated)
            .direction(Direction::Descending)
            .send()
            .await?
            .into_stream(&octocrab);

        pin!(stream);

        let pulls = Ledger::candidate_pulls(stream, &period).await?;
        let reviews = Ledger::collect_reviews(&octocrab, pull_handler, pulls, reviewers, &period).await?;

        let (used, limit, remaining) = rate_allowance(&octocrab, &owner, &repo).await?;
        log::debug!("GitHub API rate limit: {used}/{limit} used ({remaining} remaining)");

        Ok(Self { reviews })
    }

    /// Review counts per reviewer, sorted in descending order, untieing by name
    pub fn sorted_counts(&self) -> Vec<(&str, usize)> {
        let mut counts: Vec<_> = self.reviews.iter()
            .map(|(reviewer, records)| (reviewer.as_str(), records.len()))
            .collect();
        counts.sort_by_cached_key(|&(reviewer, count)| (Reverse(count), reviewer.to_lowercase()));
        counts
    }
}

// private helpers
impl Ledger {
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

    async fn collect_reviews(
        octocrab: &Octocrab,
        pull_handler: PullRequestHandler<'_>,
        pulls: Vec<PullRequest>,
        reviewers: Vec<String>,
        period: &Timeframe,
    ) -> Result<HashMap<String, Vec<Record>>> {
        let mut records: HashMap<String, Vec<Record>> = HashMap::new();
        for pull in pulls {
            log::debug!("Counting reviews on #{}", pull.number);

            let pr_author = pull.user.as_ref().expect("A pull request should have an author").login.as_str();
            let comments_by_review = Ledger::comments_by_review(octocrab, &pull_handler, pull.number).await?;
            let reviews = pull_handler.list_reviews(pull.number).send().await?.into_stream(octocrab);
            pin!(reviews);

            while let Some(review) = reviews.try_next().await? {
                let Some(review_ts) = review.submitted_at else {
                    log::debug!("Skipping pending (unsubmitted) review on #{}", pull.number);
                    continue;
                };
                log::debug!("Review timestamp #{}: {}", pull.number, review_ts);

                if period.contains(review_ts) {
                    let reviewer = &review.user.as_ref().expect("A review should have been made by someone").login;
                    if reviewer != pr_author
                        && (reviewers.is_empty() || reviewers.contains(reviewer))
                        && !Ledger::is_lone_reply(&review, &comments_by_review)
                    {
                        records.entry(reviewer.clone()).or_default().push(Record::from_github(&review));
                    }
                }
            }
        }

        Ok(records)
    }

    /// Whether a review is just a single reply to an existing thread (GitHub wraps each such reply
    /// in its own bodyless COMMENTED review), rather than an actual review
    fn is_lone_reply(review: &Review, comments_by_review: &HashMap<ReviewId, Vec<Comment>>) -> bool {
        let bodyless_comment =
            review.state == Some(ReviewState::Commented)
            && review.body.as_deref().is_none_or(str::is_empty);

        bodyless_comment && matches!(
            comments_by_review.get(&review.id).map(Vec::as_slice),
            Some([comment]) if comment.in_reply_to_id.is_some()
        )
    }

    /// Fetch all inline review comments on a pull request, grouped by the review they belong to
    async fn comments_by_review(
        octocrab: &Octocrab,
        pull_handler: &PullRequestHandler<'_>,
        pull_number: u64,
    ) -> Result<HashMap<ReviewId, Vec<Comment>>> {
        let comments = pull_handler
            .list_comments(Some(pull_number))
            .per_page(100u8)
            .send()
            .await?
            .into_stream(octocrab);
        pin!(comments);

        let mut by_review: HashMap<ReviewId, Vec<Comment>> = HashMap::new();
        while let Some(comment) = comments.try_next().await? {
            if let Some(review_id) = comment.pull_request_review_id {
                by_review.entry(review_id).or_default().push(comment);
            }
        }

        Ok(by_review)
    }
}

fn build_octocrab() -> Result<Octocrab> {
    let mut builder = Octocrab::builder();

    match std::env::var("GITHUB_TOKEN") {
        Ok(token) if !token.is_empty() => {
            log::debug!("Authenticating with personal token from $GITHUB_TOKEN");
            builder = builder.personal_token(token);
        }
        Ok(_) | Err(VarError::NotPresent) =>
            log::warn!("$GITHUB_TOKEN not set; using anonymous access (60 requests/hour)"),
        Err(e) => return Err(format!("Invalid $GITHUB_TOKEN: {e}").into()),
    }

    Ok(builder.build()?)
}

async fn rate_allowance(octocrab: &Octocrab, owner: &str, repo: &str) -> Result<(String, String, String)> {
    // /rate_limit is unreliable here (tends to lag significantly, unlike headers on billed
    // requests), so pay one actual metered request to peek at the headers instead. We need
    // to use a raw _get to avoid headers being thrown away.
    let response = octocrab._get(format!("https://api.github.com/repos/{owner}/{repo}")).await?;
    let headers = response.headers();
    let header = |name| {
        headers.get(name).and_then(|v| v.to_str().ok()).unwrap_or("?").to_string()
    };

    Ok((header("x-ratelimit-used"), header("x-ratelimit-limit"), header("x-ratelimit-remaining")))
}
