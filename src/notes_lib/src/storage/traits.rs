use crate::types::{ArchiveNoteRequest, FullList, List, Note, UpdateNoteRequest};
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

pub trait NoteStore: Send + Sync + Clone + 'static {
    fn get_note(&self, id: Uuid) -> Result<Note>;
    fn get_notes(&self, owner: String) -> Result<HashMap<Uuid, Note>>;
    fn create_note(&mut self, note: Note) -> Result<()>;
    fn update_note(&mut self, update_request: UpdateNoteRequest) -> Result<()>;
    fn archive_note(&mut self, archive_request: ArchiveNoteRequest) -> Result<()>;
    fn get_lists(&self, owner: String) -> Result<HashMap<Uuid, List>>;
    fn get_full_list(&self, id: Uuid) -> Result<FullList>;
    fn store_list(&mut self, list: List) -> Result<()>;
}
