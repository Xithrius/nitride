use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(rename_all = "kebab-case")]
#[clap(author, version, about)]
/// Twitch chat in the terminal
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        #[arg(short, long)]
        list: bool,
    },
}
