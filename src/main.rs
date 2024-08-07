use clap::{arg, command, Parser, Subcommand};
mod commit;
mod config;

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

    Config {
        #[arg(long, short)]
        list: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Commit {}) => {
            commit::handle_commit();
        }
        Some(Commands::Config { list }) => {
            if *list {
                println!("{}", config::get_config())
            }
        }
        None => {}
    }
}
