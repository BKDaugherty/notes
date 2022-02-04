use crate::storage::NoteStore;
use crate::types::{
    ArchiveNoteRequest, ArchiveNoteResponse, CreateNoteRequest, CreateNoteResponse, GetNoteRequest,
    GetNoteResponse, GetNotesRequest, GetNotesResponse, Note, Tag, UpdateNoteRequest,
    UpdateNoteResponse,
};
use anyhow::{Context, Result};
use chrono;
use std::collections::HashSet;
use uuid::Uuid;

#[derive(Clone)]
pub struct RequestHandler<S> {
    pub storage: S,
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
            owner: request.owner,
            tags: match request.tags {
                Some(tags) => tags,
                None => HashSet::new(),
            },
            create_time: format!("{}", chrono::offset::Utc::now().timestamp()),
            last_update_time: format!("{}", chrono::offset::Utc::now().timestamp()),
            delete_time: None,
        };
        self.storage
            .create_note(note)
            .context("Attempting to store note")?;
        Ok(CreateNoteResponse { note_id: uuid })
    }
    fn get_note(&self, request: GetNoteRequest) -> Result<GetNoteResponse> {
        let note = self
            .storage
            .get_note(request.note_id)
            .context("Getting Note")?;
        Ok(GetNoteResponse { note })
    }

    fn get_notes(&self, request: GetNotesRequest) -> Result<GetNotesResponse> {
        let notes = self
            .storage
            .get_notes(request.owner)
            .context("Getting Notes for owner")?;
        Ok(GetNotesResponse { notes })
    }

    fn archive_note(&mut self, request: ArchiveNoteRequest) -> Result<ArchiveNoteResponse> {
        self.storage
            .archive_note(request)
            .context("getting note to update")?;
        Ok(ArchiveNoteResponse {})
    }

    fn update_note(&mut self, request: UpdateNoteRequest) -> Result<UpdateNoteResponse> {
        // Set note in storage
        self.storage
            .update_note(request)
            .context("Attempting to update note")?;
        Ok(UpdateNoteResponse {})
    }
}

pub trait NotesService: Send + Sync + Clone + 'static {
    fn create_note(&mut self, request: CreateNoteRequest) -> Result<CreateNoteResponse>;
    fn get_note(&self, request: GetNoteRequest) -> Result<GetNoteResponse>;
    fn get_notes(&self, request: GetNotesRequest) -> Result<GetNotesResponse>;
    fn update_note(&mut self, request: UpdateNoteRequest) -> Result<UpdateNoteResponse>;
    fn archive_note(&mut self, request: ArchiveNoteRequest) -> Result<ArchiveNoteResponse>;
}
