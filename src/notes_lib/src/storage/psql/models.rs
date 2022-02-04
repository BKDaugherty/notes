use crate::storage::psql::schema::notes;
use crate::types::{ArchiveNoteRequest, Note, Tag, UpdateNoteRequest};
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

#[derive(AsChangeset, Default)]
#[table_name = "notes"]
pub struct UpdateNote {
    pub last_update_time: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub delete_time: Option<String>,
}

impl TryFrom<UpdateNoteRequest> for UpdateNote {
    type Error = anyhow::Error;
    fn try_from(request: UpdateNoteRequest) -> Result<Self, Self::Error> {
        let tag_update = match request.tags {
            Some(tag_set) => {
                let mut tags = Vec::new();
                for tag in tag_set {
                    tags.push(serde_json::to_string(&tag).context("serializing tag")?);
                }
                Some(tags)
            }
            None => None,
        };
        Ok(Self {
            last_update_time: format!("{}", chrono::offset::Utc::now().timestamp()),
            title: request.title,
            description: request.description,
            tags: tag_update,
            delete_time: None,
        })
    }
}

impl From<ArchiveNoteRequest> for UpdateNote {
    fn from(_request: ArchiveNoteRequest) -> Self {
        let now = format!("{}", chrono::offset::Utc::now().timestamp());
        Self {
            last_update_time: now.clone(),
            delete_time: Some(now),
            ..Self::default()
        }
    }
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
