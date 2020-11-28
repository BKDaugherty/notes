use crate::lib::storage::NoteStore;
use crate::lib::types::{CreateNoteRequest, CreateNoteResponse, UpdateNoteRequest, UpdateNoteResponse, GetNoteRequest, GetNoteResponse, Note};
use anyhow::{Context, Result};
use chrono;
use std::collections::HashSet;
use uuid::Uuid;


#[derive(Clone)]
pub struct RequestHandler<S> {
    pub storage: S
}

impl<S: NoteStore> RequestHandler<S> {
    pub fn new(storage: S) -> RequestHandler<S> {
        RequestHandler { storage }
    }
}

impl<S: NoteStore> NotesService for RequestHandler<S> {
    fn create_note(&mut self, request: CreateNoteRequest) -> Result<CreateNoteResponse> {
	let uuid = Uuid::new_v4();
	let note = Note {
	    uuid: uuid.clone(),
	    title: request.title,
	    description: request.description,
	    origin: request.origin,
	    tags: match request.tags {
		Some(tags) => tags,
		None => HashSet::new(),
	    },
	    create_time: format!("{}", chrono::offset::Utc::now().timestamp()),
	    last_update_time:format!("{}", chrono::offset::Utc::now().timestamp()),
	    delete_time: None,
	    
	};
	self.storage.store_note(note).context("Attempting to store note")?;
	Ok(CreateNoteResponse {
	    note_id: uuid,
	})
	
    }
    fn get_note(&self, request: GetNoteRequest) -> Result<GetNoteResponse> {
	let note = self.storage.get_note(request.note_id).context("Getting Note")?;
	Ok(GetNoteResponse {
	    note
	})
    }
    fn update_note(&mut self, request: UpdateNoteRequest) -> Result<UpdateNoteResponse> {
	self.storage.store_note(request.note).context("Attempting to update note")?;
	Ok(UpdateNoteResponse {})
    }
}


pub trait NotesService : Send + Sync + Clone + 'static {
    fn create_note(&mut self, request: CreateNoteRequest) -> Result<CreateNoteResponse>;
    fn get_note(&self, request: GetNoteRequest) -> Result<GetNoteResponse>;
    fn update_note(&mut self, request: UpdateNoteRequest) -> Result<UpdateNoteResponse>;
}



