use clap::{Parser, ValueEnum};
#[derive(Parser)]
pub struct Cli {
    /// type of movie to display
    #[arg(short, long)]
    pub r#type: Mode
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    PLAYING,
    POPULAR,
    UPCOMING,
    TOP
}