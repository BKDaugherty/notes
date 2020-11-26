use uuid::Uuid;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Note {
    pub uuid: Uuid,
    pub name: String,
}
