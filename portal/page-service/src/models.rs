
use diesel::types::{Timestamp};

#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: Timestamp,
    pub updated_at: Timestamp
}
