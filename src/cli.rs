use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// run check, update status for all
    Check,
    // list articles out of date
    // List {
    //     /// force listing of all articles
    //     #[arg(short, long)]
    //     all: bool,
    // },
    /// fix article, mark up to date
    Fix {
        #[arg(short, long)]
        article: String,
    },
    // ensure badge for articles
    // EnsureBadge {
    //     /// ensure for single article
    //     #[arg(short, long)]
    //     article: Option<String>,
    // },
}
