use std::{env, fs::{self}};

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;


#[tokio::main]
async fn main() {

    dotenv().expect("Cannot read .env file");

    let db_url = env::var("DB_URL").expect("DB_URL environment variable not found!");

    let args: Vec<String> = env::args().collect();

    assert!(args.len() == 3, "Command should be in format 'cmd <title> <path/to/content>'");

    let body: String = fs::read_to_string(&args[2]).expect(&format!("Could not read {}", &args[2]));

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&db_url)
        .await
        .expect("could not create db pool");

    let _result = sqlx::query("INSERT INTO myposts (post_title, post_body) VALUES ($1, $2)")
        .bind(&args[1])
        .bind(body)
        .execute(&pool)
        .await
        .expect("Failed to insert into db");
}