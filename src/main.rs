use clap::Parser;
use log::LevelFilter;
use review_stats::{repo_instance::RepoInstance,
                   reviews::Stats,
                   simple_logger::SimpleLogger,
                   timeframe::Timeframe};

#[derive(Parser)]
#[command(
    author = "Ricardo Abreu, ricab@ricabhome.org",
    about = "Count reviews on a GitHub repository",
    arg_required_else_help = true,
    after_help = "Examples:
  review-stats octocat/Hello-World
  review-stats --reviewers alice,bob microsoft/vscode
  review-stats --period 2w canonical/multipass"
)]
struct Args {
    /// GitHub repository in format owner/repo
    repository: RepoInstance,

    /// Optional comma-separated reviewer usernames to filter for (e.g. alice,bob)
    /// [default: no filter]
    #[arg(long, value_delimiter = ',')]
    reviewers: Vec<String>,

    /// Optional time period to analyze (e.g. 10d, 2w, 3m, 24h)
    #[arg(long, default_value = Self::DEFAULT_PERIOD)]
    period: Timeframe,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

impl Args {
    const DEFAULT_PERIOD: &'static str = "15d";
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    SimpleLogger::init(if args.verbose { LevelFilter::Debug } else { LevelFilter::Warn });

    let stats = Stats::build(args.repository, args.reviewers, args.period).await;
    println!("{stats:?}");
}
