use clap::{Parser, ValueEnum};
#[derive(Parser)]
pub struct Cli {
    /// type of movie to display
    #[arg(short, long)]
    r#type: Mode
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    PLAYING,
    POPULAR,
    UPCOMING,
    TOP
}