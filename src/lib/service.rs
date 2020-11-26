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
    fn create_note(&self, note: Note) -> Result<()> {
        todo!()
    }
    fn get_note(&self, note_id: Uuid) -> Result<Note> {
        todo!()
    }
    fn update_note(&self, note_id: Uuid, note: Note) -> Result<()> {
        todo!()
    }
}

pub trait NotesService {
    fn create_note(&self, note: Note) -> Result<()>;
    fn get_note(&self, note_id: Uuid) -> Result<Note>;
    fn update_note(&self, note_id: Uuid, note: Note) -> Result<()>;
}



