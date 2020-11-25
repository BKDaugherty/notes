use warp::{filters::BoxedFilter, Filter, Reply};

pub fn get_routes() -> BoxedFilter<(impl Reply,)> {
    warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name))
        .boxed()
}
