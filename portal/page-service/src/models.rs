use diesel::types::*;
use std::time::{SystemTime};

#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: SystemTime,
    pub updated_at: SystemTime
}
