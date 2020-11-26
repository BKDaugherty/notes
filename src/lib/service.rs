use crate::lib::storage::NoteStore;
use crate::lib::types::Note;
use anyhow::Result;
use uuid::Uuid;


pub struct RequestHandler {
    pub storage: Box<dyn NoteStore>,
}

impl RequestHandler {
    pub fn new(storage: Box<dyn NoteStore>) -> RequestHandler {
        RequestHandler { storage }
    }
}

impl NotesService for RequestHandler {
    fn create_note(&mut self, note: Note) -> Result<()> {
	self.storage.store_note(note)
    }
    fn get_note(&self, note_id: Uuid) -> Result<Note> {
	self.storage.get_note(note_id)
    }
    fn update_note(&mut self, _note_id: Uuid, note: Note) -> Result<()> {
	self.storage.store_note(note)
    }
}

pub trait NotesService {
    fn create_note(&mut self, note: Note) -> Result<()>;
    fn get_note(&self, note_id: Uuid) -> Result<Note>;
    fn update_note(&mut self, note_id: Uuid, note: Note) -> Result<()>;
}



