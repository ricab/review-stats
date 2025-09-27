use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Ricardo Abreu, ricab@ricabhome.org",
    about = "Count reviews on a GitHub repository"
)]
struct Args {
    /// GitHub repository in format owner/repo
    repo: String,
}

fn main() {
    let args = Args::parse();
    println!("Repository: {}", args.repo);
}
