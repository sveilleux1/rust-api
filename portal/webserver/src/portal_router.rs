extern crate page_api;

use router::Router;

pub fn initialize(router: &mut Router) {
    page_api::mount_api(router);
}
