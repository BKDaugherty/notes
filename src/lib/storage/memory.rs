use super::traits::NoteStore;
use crate::lib::types::Note;
use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

pub struct MemoryNoteStore {
    storage: HashMap<Uuid, Note>,
}

impl MemoryNoteStore {
    pub fn new() -> MemoryNoteStore {
        MemoryNoteStore {
            storage: HashMap::new(),
        }
    }
}

impl NoteStore for MemoryNoteStore {
    fn get_note(&self, _id: Uuid) -> Result<Note> {
        Ok(Note::default())
    }

    fn store_note(&self, _note: Note) -> Result<()> {
        Ok(())
    }
}
