use sqlx::prelude::FromRow;
use sqlx::types::time::Date;

#[derive(FromRow, Debug, Clone)]
pub struct Post {
    pub title: String,
    pub date: Date,
    pub body: String,
    pub tags: Vec<String>,
}
