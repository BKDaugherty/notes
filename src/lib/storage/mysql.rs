use super::traits::NoteStore;
use crate::lib::types::Note;
use anyhow::{Context, Result};
use mysql_async::Pool;
use uuid::Uuid;

pub struct MysqlNoteStore {
    connection_pool: Pool,
}

impl MysqlNoteStore {
    pub fn new(db_url: String) -> Result<MysqlNoteStore> {
        let connection_pool = mysql_async::Pool::from_url(db_url).context("Creating mysql pool")?;
        Ok(MysqlNoteStore { connection_pool })
    }
}

impl NoteStore for MysqlNoteStore {
    fn get_note(&self, _id: Uuid) -> Result<Note> {
        Ok(Note::default())
    }

    fn store_note(&mut self, _note: Note) -> Result<()> {
        Ok(())
    }
}
