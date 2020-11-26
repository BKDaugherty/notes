use std::collections::HashSet;
use uuid::Uuid;

#[derive(Debug, Clone, Default)]
/// A note represents a concept or idea
pub struct Note {
    /// Unique id for this entry
    pub uuid: Uuid,
    /// The title of this entry
    pub title: String,
    /// Optional description about this entry
    pub description: Option<String>,
    /// Where you heard about this
    pub origin: Option<String>,
    /// Tags assciated with this content
    pub tags: HashSet<Tag>,
    pub create_time: Option<String>,
    pub last_update_time: Option<String>,
    pub delete_time: Option<String>
}

/// A list or collection of notes can be used to prioritize
/// or collect various things into a group.
/// Common examples are
#[derive(Debug, Clone)]
pub struct List {
    pub uuid: Uuid,
    /// Vector of Uuid's to different Notes in the list
    pub notes: Vec<Uuid>,
    /// Title of the list
    pub title: String,
    /// Optional text that can provide any other information you'd like about this list
    pub description: Option<String>
}

/// List of tags that can be associated with a Note
/// This is explicitly encoded as a rust enum, because I don't
/// want it to be easily added to. Keeping a limited set of tags is important
#[derive(Debug, Clone)]
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

    // Meta based
    RecommendedBy(String),
    RemindsMeOf(String),
}
