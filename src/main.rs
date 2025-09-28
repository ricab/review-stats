use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Ricardo Abreu, ricab@ricabhome.org",
    about = "Count reviews on a GitHub repository",
    arg_required_else_help = true,
    after_help = "Examples:
  count-reviews octocat/Hello-World
  count-reviews --users alice,bob microsoft/vscode
  count-reviews --period 2w canonical/multipass"
)]
struct Args {
    /// GitHub repository in format owner/repo
    repo: String,

    /// Comma-separated reviewer usernames to count reviews for (e.g. alice,bob)
    #[arg(long, value_delimiter = ',')]
    reviewers: Option<Vec<String>>,

    /// Time period to analyze (e.g., 10d, 2w, 3m, 24h)
    #[arg(long)]
    period: Option<String>,
}

fn main() {
    let args = Args::parse();
    println!("Repository: {}", args.repo);

    if let Some(reviewers) = args.reviewers {
        println!("Users: {}", reviewers.join(" "));
    }

    if let Some(period) = args.period {
        println!("Period: {}", period);
    }
}
