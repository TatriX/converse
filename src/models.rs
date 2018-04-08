use chrono::prelude::{DateTime, Utc};

#[derive(Queryable, Serialize)]
pub struct Thread {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub posted: DateTime<Utc>,
}

#[derive(Queryable, Serialize)]
pub struct Post {
    pub id: i32,
    pub thread: i32,
    pub body: String,
    pub posted: DateTime<Utc>,
}
