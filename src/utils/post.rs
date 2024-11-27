use sqlx::prelude::FromRow;
use sqlx::types::time::Date;

#[derive(FromRow, Debug, Clone)]
pub struct Post {
    pub post_title: String,
    pub post_date: Date,
    pub post_body: String,
}
