use super::traits::NoteStore;
use crate::lib::types::{FullList, List, Note};
use anyhow::{Context, Result};
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager, PoolError };
use diesel::r2d2;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use uuid::Uuid;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBCon = PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool(psql_str: &str) -> std::result::Result<DBPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(psql_str);
    Pool::builder()
        .build(manager)
}

// TODO Make this async
pub fn get_db_con(db_pool: &DBPool) -> Result<DBCon, PoolError> {
    db_pool.get()
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
}

impl NoteStore for PsqlNoteStore {
    fn get_note(&self, id: Uuid) -> Result<Note> {
        todo!()
    }

    fn get_notes(&self, owner: String) -> Result<HashMap<Uuid, Note>> {
        todo!()
    }

    fn store_note(&mut self, note: Note) -> Result<()> {
        todo!()
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
