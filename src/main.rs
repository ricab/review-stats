use clap::Parser;
use review_stats::{repo_instance::RepoInstance, reviews::Stats, timeframe::Timeframe};

#[derive(Parser)]
#[command(
    author = "Ricardo Abreu, ricab@ricabhome.org",
    about = "Count reviews on a GitHub repository",
    arg_required_else_help = true,
    after_help = "Examples:
  count-reviews octocat/Hello-World
  count-reviews --reviewers alice,bob microsoft/vscode
  count-reviews --period 2w canonical/multipass"
)]
struct Args {
    /// GitHub repository in format owner/repo
    repo: String, // TODO@ricab add custom type to parse repo into owner/repo struct

    /// Optional comma-separated reviewer usernames to filter for (e.g. alice,bob)
    /// [default: no filter]
    #[arg(
        long,
        value_delimiter = ',',
        default_value = Args::DEFAULT_REVIEWERS,
        hide_default_value = true,
    )]
    reviewers: Vec<String>,

    /// Optional time period to analyze (e.g. 10d, 2w, 3m, 24h)
    #[arg(long, default_value = Self::DEFAULT_PERIOD)]
    period: Timeframe,
}

impl Args {
    const DEFAULT_REVIEWERS: &'static str = "";
    const DEFAULT_PERIOD: &'static str = "15d";
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let stats = Stats::build(args.repo, args.reviewers, args.period).await;
    println!("{stats:?}");
}
