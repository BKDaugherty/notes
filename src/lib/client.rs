use crate::lib::types::{GetNotesRequest, GetNotesResponse};
use anyhow::Result;
use async_trait::async_trait;

// Currently the client is only used for importing my notes into Notion, so all I've implemented is get_notes.
#[async_trait]
trait Client {
    async fn get_notes(&self, request: GetNotesRequest) -> Result<GetNotesResponse>;
}

impl NotesClient {
    pub fn new(endpoint: String) -> NotesClient {
        NotesClient { endpoint }
    }
}

pub struct NotesClient {
    endpoint: String,
}

#[async_trait]
impl Client for NotesClient {
    async fn get_notes(&self, request: GetNotesRequest) -> Result<GetNotesResponse> {
        let resp = reqwest::get(format!("{}/notes/{}", self.endpoint, request.owner))
            .await?
            .json::<GetNotesResponse>()
            .await;
        Ok(resp?)
    }
}
