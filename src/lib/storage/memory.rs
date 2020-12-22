use super::traits::NoteStore;
use crate::lib::types::{List, Note};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};
use uuid::Uuid;

#[derive(Clone)]
pub struct MemoryNoteStore {
    note_storage: Arc<RwLock<HashMap<Uuid, Note>>>,
    list_storage: Arc<RwLock<HashMap<Uuid, List>>>,
}

impl MemoryNoteStore {
    pub fn new() -> MemoryNoteStore {
        MemoryNoteStore {
            note_storage: Arc::new(RwLock::new(HashMap::new())),
            list_storage: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl NoteStore for MemoryNoteStore {
    fn get_note(&self, id: Uuid) -> Result<Note> {
        self.note_storage
            .read()
            .unwrap()
            .get(&id)
            .context(format!("Looking for note with id {}", id))
            .map(|x| x.clone())
    }

    fn get_notes(&self, owner: String) -> Result<HashMap<Uuid, Note>> {
        let mut map = HashMap::new();
        let storage = self.note_storage.read().unwrap();
        for note in storage.values() {
            if note.owner == owner {
                map.insert(note.uuid.clone(), note.clone());
            }
        }
        Ok(map)
    }

    fn store_note(&mut self, note: Note) -> Result<()> {
        self.note_storage
            .write()
            .unwrap()
            .insert(note.uuid.clone(), note);
        Ok(())
    }
}
