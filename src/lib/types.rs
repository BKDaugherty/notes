use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
/// A note represents a concept or idea
pub struct Note {
    /// Unique id for this entry
    pub uuid: Uuid,
    /// The title of this entry
    pub title: String,
    /// The owner of this note
    pub owner: String,
    /// Optional description about this entry
    pub description: Option<String>,
    /// Where you heard about this
    pub origin: Option<String>,
    /// Tags assciated with this content
    pub tags: HashSet<Tag>,
    pub create_time: String,
    pub last_update_time: String,
    pub delete_time: Option<String>,
}


/// A list or collection of notes can be used to prioritize
/// or collect various things into a group.
/// Common examples are
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct List {
    pub uuid: Uuid,
    /// Vector of Uuid's to different Notes in the list
    pub notes: Vec<Uuid>,
    /// Title of the list
    pub title: String,
    /// The owner of this list
    pub owner: String,
    /// Optional text that can provide any other information you'd like about this list
    pub description: Option<String>,
}

/// List of tags that can be associated with a Note
/// This is explicitly encoded as a rust enum, because I don't
/// want it to be easily added to. Keeping a limited set of tags is important
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Tag {
    // Medium Based
    Article,
    Book,
    Movie,
    Music,

    // Genre based
    Career,
    Entertainment,
    Productivity,

    // Topic based
    ArtificialIntelligence,
    EffectiveAltruism,
    SocialJustice,

    // Meta based
    RecommendedBy(String),
    RemindsMeOf(String),
}

// API Interface
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub description: Option<String>,
    pub origin: Option<String>,
    pub tags: Option<HashSet<Tag>>,
    pub owner: String,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct CreateNoteResponse {
    pub note_id: Uuid,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNoteRequest {
    pub note_id: Uuid
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNotesRequest {
    pub owner : String
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNotesResponse {
    pub notes: HashMap<Uuid, Note>
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct GetNoteResponse {
    pub note: Note
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNoteRequest {
    /// note to update
    pub note_id: Uuid,
    pub title: Option<String>,
    pub description: Option<String>,
    pub origin: Option<String>,
    pub tags: Option<HashSet<Tag>>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct UpdateNoteResponse {
    
}
