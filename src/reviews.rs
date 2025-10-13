use crate::{timeframe::Timeframe, Result};

/// Statistics on GitHub reviews
#[derive(Debug)]
pub struct Stats;

impl Stats {
    pub fn build(
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

        Ok(Self {})
    }
}
