use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: ClientCommand,
}

#[derive(Subcommand, Debug)]
pub enum ClientCommand {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String
    }
}