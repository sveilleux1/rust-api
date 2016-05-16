use page_service::PageService;
use iron::prelude::*;
use iron::status;

use std::sync::{Mutex, Arc};
use diesel::pg::PgConnection;

pub struct Controller<'a> {
    database_connection: Arc<Mutex<&'a PgConnection>>
    //page_service: PageService<'a>
}

impl<'a> Controller<'a> {
    pub fn new(database_connection : Arc<Mutex<&'a PgConnection>>) -> Controller<'a>  {
        Controller {
            database_connection : database_connection
            //page_service: PageService::new(database_connection)
        }
    }

    pub fn get_pages(&mut self, request: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "allo".to_string())))
        //let pages = self.page_service.get_pages();
    }
}
