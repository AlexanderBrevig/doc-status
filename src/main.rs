mod cli;
mod docstatus;

use crate::cli::Commands;
use crate::docstatus::{DocStatus, MatrixFile};
use clap::Parser;
use cli::Cli;
use docstatus::Matrix;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // trigger build
    env_logger::init();
    let cli = Cli::parse();

    let default_config_file = PathBuf::from(".doc-status.json");
    let config = cli.config.as_deref().unwrap_or(&default_config_file);

    let mut doc_status = DocStatus::from(config);

    match &cli.command {
        Some(Commands::Check) => {
            let changed_files = doc_status.check()?;
            // {"include":[{"file":"file1"},{"file":"file2"}]}
            let mut matrix = Matrix { include: vec![] };
            for file in changed_files {
                matrix.include.push(MatrixFile { file });
            }
            let matrix = serde_json::to_string(&matrix)?;
            println!("{matrix}");
        }
        Some(Commands::List { all }) => {
            let list = doc_status.list(all);
            println!("{:?}", list);
        }
        Some(Commands::Fix { article }) => {
            let fix = doc_status.fix(article);
            println!("{:?}", fix);
        }
        Some(Commands::EnsureBadge { article }) => {
            let ensure = doc_status.ensure_badge(article);
            println!("{:?}", ensure);
        }
        None => {}
    }
    let file = File::create(config).unwrap();
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, &doc_status)?;
    writer.flush()?;
    Ok(())
}
