use std::{env, fs, sync::OnceLock};

use sqlx::postgres::PgPoolOptions;

use crate::utils::post::Post;

static DB_URL: OnceLock<String> = OnceLock::new();

fn fetch_db_url() -> String {
    env::var("DB_URL").expect("No DB_URL environment variable!")
}

pub async fn add_post_to_db(title: &str, post_path: &str) -> anyhow::Result<()> {
    let db_url = env::var("DB_URL").expect("No DB_URL environment variable!");

    let body: String = fs::read_to_string(post_path)?;

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await?;

    let _result = sqlx::query("INSERT INTO posts (title, body) VALUES ($1, $2)")
        .bind(title)
        .bind(body)
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

    let posts = sqlx::query_as::<_, Post>("SELECT title, date, body FROM posts")
        .fetch_all(&pool)
        .await?;

    Ok(posts)
}
