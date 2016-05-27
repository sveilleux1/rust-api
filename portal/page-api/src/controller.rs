use page_service::PageService;
use iron::prelude::*;
use iron::status;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;

pub struct Controller {
    page_service: PageService
}

impl Controller {
    pub fn new(database_connection_pool : Pool<ConnectionManager<PgConnection>>) -> Controller {
        Controller {
            page_service: PageService::new(database_connection_pool)
        }
    }

    pub fn get_pages(&mut self, request: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "allo".to_string())))
        //let pages = self.page_service.get_pages();
    }
}
