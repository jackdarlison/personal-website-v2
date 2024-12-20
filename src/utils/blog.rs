use std::{env, fs::{self}, path::PathBuf, sync::OnceLock};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;

use crate::utils::post::Post;


static DB_URL: OnceLock<String> = OnceLock::new();

fn fetch_db_url() -> String {
    env::var("DB_URL").expect("No DB_URL environment variable!")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostMetadata {
    title: String,
    tags: Vec<String>,
    date: sqlx::types::time::Date,
}

pub async fn add_post_to_db(post_dir: PathBuf) -> anyhow::Result<()> {
    let db_url = DB_URL.get_or_init(fetch_db_url);

    let body: String = fs::read_to_string(post_dir.join("body.md"))?;
    let metadata_file = fs::read_to_string(post_dir.join("metadata.yaml"))?;
    let metadata: PostMetadata = serde_yaml::from_str(&metadata_file)?;

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await?;

    let _result = sqlx::query("INSERT INTO posts (date, title, body, tags) VALUES ($1, $2, $3, $4)")
        .bind(metadata.date)
        .bind(metadata.title)
        .bind(body)
        .bind(metadata.tags)
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn get_posts() -> anyhow::Result<Vec<Post>> {
    let db_url = DB_URL.get_or_init(fetch_db_url);

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await?;

    let mut posts = sqlx::query_as::<_, Post>("SELECT title, date, body, tags FROM posts")
        .fetch_all(&pool)
        .await?;

    posts.sort_by_key(|p| p.date);
    posts.reverse();

    Ok(posts)
}

pub async fn generate_posts() -> anyhow::Result<()> {

    for file in fs::read_dir("posts")? {
        let post_dir = file?;

        assert!(post_dir.file_type()?.is_dir());

        add_post_to_db(post_dir.path()).await?;
    }

    Ok(())
}
