#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use std::sync::{Mutex, Arc};
use diesel::pg::PgConnection;

pub struct PageService<'a> {
    database_connection: Arc<Mutex<&'a PgConnection>>
}

impl<'a> PageService<'a> {
    pub fn new(database_connection : Arc<Mutex<&'a PgConnection>>) -> PageService<'a> {
        PageService {
            database_connection: database_connection
        }
    }

    pub fn get_pages(&self) -> String {
        return "mes pages".to_string();
    }
}
