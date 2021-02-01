use super::models::{DBNote, NewNote, UpdateNote};
use super::schema::notes;
use crate::lib::storage::traits::NoteStore;
use crate::lib::types::{ArchiveNoteRequest, FullList, List, Note, UpdateNoteRequest};
use anyhow::{anyhow, Context, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::info;
use std::collections::HashMap;
use std::convert::TryFrom;
use uuid::Uuid;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBCon = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool(psql_str: &str) -> std::result::Result<DBPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(psql_str);
    Pool::builder().build(manager)
}

#[derive(Clone)]
pub struct PsqlNoteStore {
    db_pool: DBPool,
}

impl PsqlNoteStore {
    pub fn new(psql_str: &str) -> PsqlNoteStore {
        let db_pool = create_pool(psql_str).expect("Could not connect to database");
        PsqlNoteStore { db_pool }
    }

    pub fn get_db_conn(&self) -> Result<DBCon, PoolError> {
        self.db_pool.get()
    }
}

impl NoteStore for PsqlNoteStore {
    fn get_note(&self, id: Uuid) -> Result<Note> {
        let conn = self.get_db_conn()?;
        info!("Looking for note {}", id);
        let mut db_notes = notes::dsl::notes
            .filter(notes::dsl::uuid.eq(id.to_string()))
            .load::<DBNote>(&conn)
            .context(format!("Looking for note with id {}", id))?;

        if db_notes.len() > 1 {
            return Err(anyhow!("Multiple notes found for uuid {}", id));
        }
        match db_notes.pop() {
            Some(db_note) => Note::try_from(db_note).context("reading db note"),
            None => Err(anyhow!("No note found for id {}", id)),
        }
    }

    fn get_notes(&self, owner: String) -> Result<HashMap<Uuid, Note>> {
        let conn = self.get_db_conn()?;
        let db_notes = notes::dsl::notes
            .filter(notes::dsl::owner.eq(&owner))
            .load::<DBNote>(&conn)
            .context(format!("Looking for owner {}", owner))?;
        let mut resulting_map = HashMap::new();
        let notes: Vec<Note> = db_notes
            .into_iter()
            .map(|db_note| Note::try_from(db_note))
            .collect::<Result<Vec<Note>>>()
            .context(format!("Reading notes for {}", owner))?;
        // TODO Stop the attack of the clones
        for note in notes {
            resulting_map.insert(note.uuid.clone(), note.clone());
        }
        Ok(resulting_map)
    }

    fn create_note(&mut self, note: Note) -> Result<()> {
        let conn = self.get_db_conn()?;
        let note_uuid = note.uuid.clone();
        let new_note_request = NewNote::try_from(note).context(format!(
            "attempting to create insert statement for note with uuid {}",
            note_uuid
        ))?;
        diesel::insert_into(notes::table)
            .values(&new_note_request)
            .execute(&conn)
            .context(format!(
                "Error attempting to persist note in db with uuid {}",
                note_uuid
            ))?;
        Ok(())
    }

    fn update_note(&mut self, request: UpdateNoteRequest) -> Result<()> {
        let note_id = request.note_id.clone();
        let update = UpdateNote::try_from(request).context("converting update request")?;
        let conn = self.get_db_conn()?;
        diesel::update(notes::dsl::notes.filter(notes::dsl::uuid.eq(note_id.to_string())))
            .set(&update)
            .execute(&conn)
            .context("Updating note")?;
        Ok(())
    }

    fn archive_note(&mut self, request: ArchiveNoteRequest) -> Result<()> {
        let note_id = request.note_id.clone();
        let update = UpdateNote::from(request);
        let conn = self.get_db_conn()?;
        diesel::update(notes::dsl::notes.filter(notes::dsl::uuid.eq(note_id.to_string())))
            .set(&update)
            .execute(&conn)
            .context("Updating note")?;
        Ok(())
    }

    fn get_full_list(&self, id: Uuid) -> Result<FullList> {
        todo!()
    }

    fn get_lists(&self, owner: String) -> Result<HashMap<Uuid, List>> {
        todo!()
    }

    fn store_list(&mut self, list: List) -> Result<()> {
        todo!()
    }
}
