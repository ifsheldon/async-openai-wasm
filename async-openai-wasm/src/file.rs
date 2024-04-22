use serde::Serialize;

use crate::{
    error::OpenAIError,
    types::{CreateFileRequest, DeleteFileResponse, ListFilesResponse, OpenAIFile},
    Client,
};

/// Files are used to upload documents that can be used with features like Assistants and Fine-tuning.
pub struct Files<'c> {
    client: &'c Client,
}

impl<'c> Files<'c> {
    pub fn new(client: &'c Client) -> Self {
        Self { client }
    }

    /// Upload a file that can be used across various endpoints. The size of all the files uploaded by one organization can be up to 100 GB.
    ///
    /// The size of individual files can be a maximum of 512 MB or 2 million tokens for Assistants. See the [Assistants Tools guide](https://platform.openai.com/docs/assistants/tools) to learn more about the types of files supported. The Fine-tuning API only supports `.jsonl` files.
    ///
    /// Please [contact us](https://help.openai.com/) if you need to increase these storage limits.
    pub async fn create(&self, request: CreateFileRequest) -> Result<OpenAIFile, OpenAIError> {
        self.client.post_form("/files", request).await
    }

    /// Returns a list of files that belong to the user's organization.
    pub async fn list<Q>(&self, query: &Q) -> Result<ListFilesResponse, OpenAIError>
        where
            Q: Serialize + ?Sized,
    {
        self.client.get_with_query("/files", query).await
    }

    /// Returns information about a specific file.
    pub async fn retrieve(&self, file_id: &str) -> Result<OpenAIFile, OpenAIError> {
        self.client.get(format!("/files/{file_id}").as_str()).await
    }

    /// Delete a file.
    pub async fn delete(&self, file_id: &str) -> Result<DeleteFileResponse, OpenAIError> {
        self.client
            .delete(format!("/files/{file_id}").as_str())
            .await
    }

    /// Returns the contents of the specified file
    pub async fn retrieve_content(&self, file_id: &str) -> Result<String, OpenAIError> {
        self.client
            .get(format!("/files/{file_id}/content").as_str())
            .await
    }
}
