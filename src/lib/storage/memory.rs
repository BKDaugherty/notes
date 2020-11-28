use super::traits::NoteStore;
use crate::lib::types::Note;
use anyhow::{Context, Result};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryNoteStore {
    storage: Arc<RwLock<HashMap<Uuid, Note>>>,
}

impl MemoryNoteStore {
    pub fn new() -> MemoryNoteStore {
        MemoryNoteStore {
            storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl NoteStore for MemoryNoteStore {
    fn get_note(&self, id: Uuid) -> Result<Note> {
	self.storage.read().unwrap().get(&id).context(format!("Looking for id {}", id)).map(|x| x.clone())
    }
    
    fn store_note(&mut self, note: Note) -> Result<()> {
	self.storage.write().unwrap().insert(note.uuid.clone(), note);
        Ok(())
    }
}
