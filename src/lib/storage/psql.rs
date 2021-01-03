use super::traits::NoteStore;
use crate::lib::types::{FullList, List, Note};
use anyhow::{Context, Result};
use mobc::{Connection, Pool};
use mobc_postgres::{tokio_postgres, PgConnectionManager};
use std::collections::HashMap;
use std::str::FromStr;
use std::time::Duration;
use tokio_postgres::{Config, Error, NoTls};
use uuid::Uuid;

pub type DBCon = Connection<PgConnectionManager<NoTls>>;
pub type DBPool = Pool<PgConnectionManager<NoTls>>;

// ELMOS MAGIC NUMBERS - THANKS INTERNET
const DB_POOL_MAX_OPEN: u64 = 32;
const DB_POOL_MAX_IDLE: u64 = 8;
const DB_POOL_TIMEOUT_SECONDS: u64 = 15;

pub fn create_pool(psql_str: &str) -> std::result::Result<DBPool, mobc::Error<Error>> {
    let config = Config::from_str(psql_str)?;

    let manager = PgConnectionManager::new(config, NoTls);
    Ok(Pool::builder()
        .max_open(DB_POOL_MAX_OPEN)
        .max_idle(DB_POOL_MAX_IDLE)
        .get_timeout(Some(Duration::from_secs(DB_POOL_TIMEOUT_SECONDS)))
        .build(manager))
}

pub async fn get_db_con(db_pool: &DBPool) -> Result<DBCon, mobc::Error<tokio_postgres::Error>> {
    db_pool.get().await
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
