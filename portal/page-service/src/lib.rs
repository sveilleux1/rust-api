#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::models::*;
use self::schema::pages::dsl::*;

pub mod schema;
pub mod models;

pub struct PageService {
    database_connection_pool: Pool<ConnectionManager<PgConnection>>
}

impl PageService {
    pub fn new(database_connection_pool : Pool<ConnectionManager<PgConnection>>) -> PageService {
        PageService {
            database_connection_pool : database_connection_pool
        }
    }

    pub fn get_pages(&self) -> String {
        let ref database_connection = *self.database_connection_pool.get().unwrap();

        let results = pages.filter(id.eq(1))
           .limit(5)
           .load::<Page>(database_connection)
           .expect("Error loading pages");

        println!("Displaying {} pages", results.len());
        for page in results {
            println!("{}", page.title);
            println!("----------\n");
            println!("{}", page.body);
        }

        return "mes pages".to_string();
    }
}
