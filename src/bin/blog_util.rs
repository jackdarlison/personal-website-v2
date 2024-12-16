#[allow(dead_code)]
#[allow(unused)]

use std::path::Path;

use clap::{Parser, Subcommand};
use utils::blog::add_post_to_db;

#[path = "../utils/mod.rs"]
mod utils;

#[derive(Subcommand, Debug)]
enum Modes {
    Add { name: String },
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[command(subcommand)]
    mode: Modes
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Maybe add other commands?
    let args = Args::parse();

    match args.mode {
        Modes::Add { name } => add_post_to_db(Path::new(&name).to_path_buf()).await
    }
}
