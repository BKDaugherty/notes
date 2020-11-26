use crate::lib::service::NotesService;
use crate::lib::types::Note;
use uuid::Uuid;
use warp::{filters::BoxedFilter, Filter, Reply};

/// This function links the service to warp's route handling
pub fn build_warp_routes(handler: Box<dyn NotesService>) -> BoxedFilter<(impl Reply,)> {
    // POST /notes/:note_id  Note?
    let create_note = warp::post()
        .and(warp::path("notes"))
        // Curious if it will allow this
        .and(warp::path::param::<Uuid>())
        // Only accept bodies smaller than 16kb... (because warp said so)
        // https://github.com/seanmonstar/warp/blob/master/examples/body.rs
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        // TODO Need to handle errors here in a good way
        .map(|_note_id, note: Note| warp::reply::json(&note))
        .boxed();
    create_note
}
