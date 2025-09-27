use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Ricardo Abreu, ricab@ricabhome.org",
    about = "Count reviews on a GitHub repository"
)]
struct Args {
    /// GitHub repository in format owner/repo
    repo: String,

    /// Comma-separated usernames to filter reviews for
    #[arg(long, value_delimiter = ',')]
    users: Option<Vec<String>>,
}

fn main() {
    let args = Args::parse();
    println!("Repository: {}", args.repo);

    if let Some(users) = args.users {
        println!("Users: {}", users.join(" "));
    }
}
