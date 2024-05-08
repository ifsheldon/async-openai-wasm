use crate::{
    error::OpenAIError,
    types::{CreateModerationRequest, CreateModerationResponse},
    Client,
};

/// Given some input text, outputs if the model classifies it as potentially harmful across several categories.
///
/// Related guide: [Moderations](https://platform.openai.com/docs/guides/moderation)
pub struct Moderations<'c> {
    client: &'c Client,
}

impl<'c> Moderations<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Classifies if text is potentially harmful.
    pub async fn create(
        &self,
        request: CreateModerationRequest,
    ) -> Result<CreateModerationResponse, OpenAIError> {
        self.client.post("/moderations", request).await
    }
}
