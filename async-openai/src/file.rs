use bytes::Bytes;

use crate::{
    Client,
    config::Config,
    error::OpenAIError,
    types::files::{CreateFileRequest, DeleteFileResponse, ListFilesResponse, OpenAIFile},
    Client, RequestOptions,
};

/// Files are used to upload documents that can be used with features like Assistants and Fine-tuning.
pub struct Files<'c, C: Config> {
    client: &'c Client<C>,
    pub(crate) request_options: RequestOptions,
}

impl<'c, C: Config> Files<'c, C> {
    pub fn new(client: &'c Client<C>) -> Self {
        Self {
            client,
            request_options: RequestOptions::new(),
        }
    }

    /// Upload a file that can be used across various endpoints. Individual files can be up to 512 MB, and the size of all files uploaded by one organization can be up to 1 TB.
    ///
    /// The Assistants API supports files up to 2 million tokens and of specific file types. See the [Assistants Tools guide](https://platform.openai.com/docs/assistants/tools) for details.
    ///
    /// The Fine-tuning API only supports `.jsonl` files. The input also has certain required formats for fine-tuning [chat](https://platform.openai.com/docs/api-reference/fine-tuning/chat-input) or [completions](https://platform.openai.com/docs/api-reference/fine-tuning/completions-input) models.
    ///
    /// The Batch API only supports `.jsonl` files up to 200 MB in size. The input also has a specific required [format](https://platform.openai.com/docs/api-reference/batch/request-input).
    ///
    /// Please [contact us](https://help.openai.com/) if you need to increase these storage limits.
    #[crate::byot(
        T0 = Clone,
        R = serde::de::DeserializeOwned,
        where_clause =  "reqwest::multipart::Form: crate::traits::AsyncTryFrom<T0, Error = OpenAIError>",
    )]
    pub async fn create(&self, request: CreateFileRequest) -> Result<OpenAIFile, OpenAIError> {
        self.client
            .post_form("/files", request, &self.request_options)
            .await
    }

    /// Returns a list of files that belong to the user's organization.
    #[crate::byot(R = serde::de::DeserializeOwned)]
    pub async fn list(&self) -> Result<ListFilesResponse, OpenAIError> {
        self.client.get("/files", &self.request_options).await
    }

    /// Returns information about a specific file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn retrieve(&self, file_id: &str) -> Result<OpenAIFile, OpenAIError> {
        self.client
            .get(format!("/files/{file_id}").as_str(), &self.request_options)
            .await
    }

    /// Delete a file.
    #[crate::byot(T0 = std::fmt::Display, R = serde::de::DeserializeOwned)]
    pub async fn delete(&self, file_id: &str) -> Result<DeleteFileResponse, OpenAIError> {
        self.client
            .delete(format!("/files/{file_id}").as_str(), &self.request_options)
            .await
    }

    /// Returns the contents of the specified file
    pub async fn content(&self, file_id: &str) -> Result<Bytes, OpenAIError> {
        let (bytes, _headers) = self
            .client
            .get_raw(
                format!("/files/{file_id}/content").as_str(),
                &self.request_options,
            )
            .await?;
        Ok(bytes)
    }
}
