use anyhow::{Context, Result};
use log::info;
use mysql_async::prelude::Queryable;
use mysql_async::Pool;
use uuid::Uuid;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Note {
    pub uuid: Uuid,
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
        let connection_pool = mysql_async::Pool::from_url(db_url).context("Creating mysql pool")?;
        Ok(MysqlNoteStore { connection_pool })
    }

    // temporary
    pub async fn init(&self) -> Result<()> {
        let mut conn = self
            .connection_pool
            .get_conn()
            .await
            .context("Connecting to DB")?;
        info!("Initializing DB");
        // Create table
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS notes (
              uuid VARCHAR(36) not null,
              name text not null
             )",
        )
        .await
        .context("Running create table")?;
        Ok(())
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
