use clap::{command, Parser, Subcommand};
mod commit;

#[derive(Parser)]
#[command(version, author, about)]
struct Cli {
    #[arg()]
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Commit {},
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Commit {}) => {
            commit::handle_commit();
        }
        None => {}
    }
}
