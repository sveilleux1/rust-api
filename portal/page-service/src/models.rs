use diesel::pg::data_types::*;

#[derive(Queryable)]
pub struct Page {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp
}
