use anyhow::Result;
use mysql_async::Pool;
use uuid::Uuid;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Note {
    pub id: Uuid,
    pub name: String,
}

pub trait NoteStore {
    fn get_note(id: Uuid) -> Result<Note>;
    // TODO (Library Errors)
    fn store_note(note: Note) -> Result<()>;
}

pub struct MysqlNoteStore {
    connection_pool: Pool,
}

impl MysqlNoteStore {
    pub fn new(db_url: String) -> Result<MysqlNoteStore> {
        let connection_pool = mysql_async::Pool::new(db_url);
        Ok(MysqlNoteStore { connection_pool })
    }
}

impl NoteStore for MysqlNoteStore {
    fn get_note(id: Uuid) -> Result<Note> {
        Ok(Note::default())
    }

    fn store_note(_note: Note) -> Result<()> {
        Ok(())
    }
}
