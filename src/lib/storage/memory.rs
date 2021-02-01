use super::traits::NoteStore;
use crate::lib::types::{ArchiveNoteRequest, FullList, List, Note, UpdateNoteRequest};
use anyhow::{Context, Result};
use std::collections::HashMap;
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

    fn create_note(&mut self, note: Note) -> Result<()> {
        self.note_storage
            .write()
            .unwrap()
            .insert(note.uuid.clone(), note);
        Ok(())
    }

    fn update_note(&mut self, request: UpdateNoteRequest) -> Result<()> {
        // Make Updates for all fields of UpdateNoteRequest
        // Get note from storage
        let mut note = self
            .get_note(request.note_id)
            .context("getting note to update")?;

        if let Some(title) = request.title {
            note.title = title;
        }
        if let Some(description) = request.description {
            note.description = description;
        }
        if let Some(tags) = request.tags {
            note.tags = tags.into();
        }
        note.last_update_time = format!("{}", chrono::offset::Utc::now().timestamp());
        Ok(())
    }

    fn archive_note(&mut self, archive_request: ArchiveNoteRequest) -> Result<()> {
        let mut note = self
            .get_note(archive_request.note_id)
            .context("getting note to update")?;
        note.delete_time = Some(format!("{}", chrono::offset::Utc::now().timestamp()));
        Ok(())
    }

    fn get_lists(&self, owner: String) -> Result<HashMap<Uuid, List>> {
        let mut map = HashMap::new();
        let storage = self.list_storage.read().unwrap();
        for list in storage.values() {
            if list.owner == owner {
                map.insert(list.uuid.clone(), list.clone());
            }
        }
        Ok(map)
    }
    fn get_full_list(&self, id: Uuid) -> Result<FullList> {
        let mut notes_in_list = HashMap::new();
        let list = self
            .list_storage
            .read()
            .unwrap()
            .get(&id)
            .context(format!("Looking for list with id {}", id))
            .map(|x| x.clone())?;
        for note_id in &list.notes {
            // TODO --> What happens if I have a deleted note in a list?
            let note = self
                .get_note(*note_id)
                .context("Looking for note in list")?;
            notes_in_list.insert(note_id.clone(), note);
        }
        Ok(FullList {
            list,
            notes_in_list,
        })
    }
    fn store_list(&mut self, list: List) -> Result<()> {
        self.list_storage
            .write()
            .unwrap()
            .insert(list.uuid.clone(), list);
        Ok(())
    }
}
