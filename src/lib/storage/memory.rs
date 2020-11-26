use super::traits::NoteStore;
use crate::lib::types::Note;
use anyhow::{Context, Result};
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
    fn get_note(&self, id: Uuid) -> Result<Note> {
	self.storage.get(&id).context(format!("Looking for id {}", id)).map(|x| x.clone())
    }
    
    fn store_note(&mut self, note: Note) -> Result<()> {
	self.storage.insert(note.uuid.clone(), note).context("Setting Note")?;
        Ok(())
    }
}
