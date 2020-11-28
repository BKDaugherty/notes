use crate::lib::service::{NotesService, RequestHandler};
use crate::lib::storage::NoteStore;
use crate::lib::types::{CreateNoteRequest, UpdateNoteRequest, GetNoteRequest, Note};
use uuid::Uuid;
use warp::{filters::BoxedFilter, http, Filter, Reply};

async fn create_note<S: NoteStore>(
    request: CreateNoteRequest,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = handler
        .create_note(request)
        .expect("Should be able to create note");
    Ok(warp::reply::with_status(
        format!("Uuid: {}", response.note_id),
        http::StatusCode::CREATED,
    ))
}

async fn get_note<S: NoteStore>(
    uuid: Uuid,
    handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = handler
        .get_note(GetNoteRequest { note_id: uuid })
        .expect("Should be able to get note");
    Ok(warp::reply::json(&response))
}

async fn update_note<S: NoteStore>(
    uuid: Uuid,
    note: Note,
    mut handler: RequestHandler<S>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let response = handler.update_note(
	UpdateNoteRequest { note_id: uuid, note })
	.expect("Should be able to update note");
    Ok(warp::reply::json(&response))
}

/// This function links the service to warp's route handling
pub fn build_warp_routes<S: NoteStore>(handler: RequestHandler<S>) -> BoxedFilter<(impl Reply,)> {
    // POST /notes/  Note?
    let handler_filter = warp::any().map(move || handler.clone());
    let create = warp::post()
        .and(warp::path("notes"))
        // Only accept bodies smaller than 16kb... (because warp said so)
        // https://github.com/seanmonstar/warp/blob/master/examples/body.rs
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(handler_filter.clone())
        .and_then(create_note);

    let get_note = warp::get()
        .and(warp::path("notes"))
        .and(warp::path::param::<Uuid>())
        .and(handler_filter.clone())
        .and_then(get_note);

    let update_note = warp::put()
	.and(warp::path("notes"))
        .and(warp::path::param::<Uuid>())
	.and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
	.and(handler_filter.clone())
	.and_then(update_note);

    let routes = create.or(get_note).or(update_note).boxed();
    routes
}
