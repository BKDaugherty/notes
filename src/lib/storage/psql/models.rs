use super::schema::notes;
use crate::lib::types::{Note, Tag};
use anyhow::{anyhow, Context};
use std::collections::HashSet;
use std::convert::TryFrom;
use uuid::Uuid;

/// Our DB representaiton of a note
#[derive(Queryable)]
pub struct DBNote {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub owner: String,
    pub description: String,
    pub create_time: String,
    pub last_update_time: String,
    pub delete_time: Option<String>,
    // TODO: impl Tag ToSql / FromSql
    pub tags: Vec<String>,
}

// Conversions from type to DB type. I'm leaning towards TryFrom over from to allow for
// errors, but maybe from would be better? Still gives flexibility?
impl TryFrom<DBNote> for Note {
    type Error = anyhow::Error;

    fn try_from(note: DBNote) -> Result<Self, Self::Error> {
        let mut tags = HashSet::new();
        let uuid = Uuid::parse_str(&note.uuid).context("Parsing uuid")?;
        for db_tag in note.tags {
            let tag: Tag = serde_json::from_str(&db_tag).context("Deserializing tag")?;
            if tags.contains(&tag) {
                return Err(anyhow!("Tag {:?} already exists!", tag));
            }
            tags.insert(tag);
        }
        Ok(Note {
            uuid,
            title: note.title,
            description: note.description,
            create_time: note.create_time,
            last_update_time: note.last_update_time,
            delete_time: note.delete_time,
            tags: tags,
	    owner: note.owner,
        })
    }
}

#[derive(Insertable)]
#[table_name = "notes"]
pub struct NewNote {
    pub uuid: String,
    pub title: String,
    pub description: String,
    pub create_time: String,
    pub last_update_time: String,
    pub delete_time: Option<String>,
    pub owner: String,
    pub tags: Vec<String>,
}

impl TryFrom<Note> for NewNote {
    type Error = anyhow::Error;
    fn try_from(note: Note) -> Result<Self, Self::Error> {
        let mut tags = Vec::new();
        for tag in note.tags {
            tags.push(serde_json::to_string(&tag).context("serializing tag")?);
        }
        Ok(NewNote {
            uuid: note.uuid.to_string(),
            title: note.title,
            description: note.description,
            create_time: note.create_time,
            last_update_time: note.last_update_time,
            delete_time: note.delete_time,
	    owner: note.owner,
            tags,
        })
    }
}
