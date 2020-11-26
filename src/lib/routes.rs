use crate::lib::service::NotesService;
use warp::{filters::BoxedFilter, Filter, Reply};

/// This function links the service to warp's route handling
pub fn build_warp_routes(handler: Box<dyn NotesService>) -> BoxedFilter<(impl Reply,)> {
    warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name)).boxed()
}
