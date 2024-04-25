use add::AddSubCommand;
use clap::{Parser, Subcommand};
use remove::RemoveSubCommand;
mod add;
mod remove;

#[derive(Debug, Subcommand)]
pub enum Command {
    Add(AddSubCommand),
    Remove(RemoveSubCommand),
}

#[derive(Parser)]
pub struct TodoCliOptions {
    #[clap(subcommand)]
    pub command: Command,
}
