//use page_service::PageService;
use iron::prelude::*;
use iron::status;

pub struct Controller {
    //page_service: PageService
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            //page_service: PageService
        }
    }

    pub fn get_pages(&self, request: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "allo".to_string())))
        //let pages = self.page_service.get_pages();
    }
}
