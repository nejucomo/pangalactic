use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Options {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Options {
    pub fn parse() -> Self {
        <Self as Parser>::parse()
    }
}

#[derive(Debug, Subcommand)]
pub enum Command {}
