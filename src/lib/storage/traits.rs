use crate::lib::types::Note;
use anyhow::Result;
use uuid::Uuid;

pub trait NoteStore {
    fn get_note(&self, id: Uuid) -> Result<Note>;
    fn store_note(&self, note: Note) -> Result<()>;
}
