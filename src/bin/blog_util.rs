use std::env;

use utils::blog::add_post_to_db;

#[path = "../utils/mod.rs"]
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Maybe add other commands?

    let args: Vec<String> = env::args().collect();

    assert!(
        args.len() == 3,
        "Command should be in format 'cmd <title> <path/to/content>'"
    );

    add_post_to_db(&args[1], &args[2]).await
}
