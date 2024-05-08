use std::future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};
use pin_project_lite::pin_project;
use std::rc::Rc;

use bytes::Bytes;
use futures::{stream::StreamExt, Stream};
use futures::stream::Filter;
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
    config::{Config, OpenAIConfig},
    error::{map_deserialization_error, OpenAIError, WrappedError},
    file::Files,
    image::Images,
    moderation::Moderations,
    Assistants, Audio, Batches, Chat, Completions, Embeddings, FineTuning, Models, Threads,
    VectorStores,
};

#[derive(Debug, Clone)]
/// Client is a container for config, backoff and http_client
/// used to make API calls.
pub struct Client {
    http_client: reqwest::Client,
    config: Rc<dyn Config>,
    // #[cfg(feature = "backoff")]
    // backoff: backoff::ExponentialBackoff,
}

impl Client {
    /// Client with default [OpenAIConfig]
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config: Rc::new(OpenAIConfig::default()),
            // #[cfg(feature = "backoff")]
            // backoff: Default::default(),
        }
    }
}

impl Client {
    /// Create client with a custom HTTP client, OpenAI config, and backoff.
    pub fn build<C: Config>(
        http_client: reqwest::Client,
        config: C,
        // #[cfg(feature = "backoff")]
        // backoff: backoff::ExponentialBackoff,
    ) -> Self {
        Self {
            http_client,
            config: Rc::new(config),
            // #[cfg(feature = "backoff")]
            // backoff,
        }
    }

    /// Create client with [OpenAIConfig] or [crate::config::AzureConfig]
    pub fn with_config<C: Config>(config: C) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config: Rc::new(config),
            // #[cfg(feature = "backoff")]
            // backoff: Default::default(),
        }
    }

    /// Provide your own [client] to make HTTP requests with.
    ///
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    // #[cfg(feature = "backoff")]
    // /// Exponential backoff for retrying [rate limited](https://platform.openai.com/docs/guides/rate-limits) requests.
    // pub fn with_backoff(mut self, backoff: backoff::ExponentialBackoff) -> Self {
    //     self.backoff = backoff;
    //     self
    // }

    // API groups

    /// To call [Models] group related APIs using this client.
    pub fn models(&self) -> Models {
        Models::new(self)
    }

    /// To call [Completions] group related APIs using this client.
    pub fn completions(&self) -> Completions {
        Completions::new(self)
    }

    /// To call [Chat] group related APIs using this client.
    pub fn chat(&self) -> Chat {
        Chat::new(self)
    }

    /// To call [Images] group related APIs using this client.
    pub fn images(&self) -> Images {
        Images::new(self)
    }

    /// To call [Moderations] group related APIs using this client.
    pub fn moderations(&self) -> Moderations {
        Moderations::new(self)
    }

    /// To call [Files] group related APIs using this client.
    pub fn files(&self) -> Files {
        Files::new(self)
    }

    /// To call [FineTuning] group related APIs using this client.
    pub fn fine_tuning(&self) -> FineTuning {
        FineTuning::new(self)
    }

    /// To call [Embeddings] group related APIs using this client.
    pub fn embeddings(&self) -> Embeddings {
        Embeddings::new(self)
    }

    /// To call [Audio] group related APIs using this client.
    pub fn audio(&self) -> Audio {
        Audio::new(self)
    }

    /// To call [Assistants] group related APIs using this client.
    pub fn assistants(&self) -> Assistants {
        Assistants::new(self)
    }

    /// To call [Threads] group related APIs using this client.
    pub fn threads(&self) -> Threads {
        Threads::new(self)
    }

    /// To call [VectorStores] group related APIs using this client.
    pub fn vector_stores(&self) -> VectorStores {
        VectorStores::new(self)
    }

    /// To call [Batches] group related APIs using this client.
    pub fn batches(&self) -> Batches {
        Batches::new(self)
    }

    pub fn config(&self) -> &Rc<dyn Config> {
        &self.config
    }

    /// Make a GET request to {path} and deserialize the response body
    pub(crate) async fn get<O>(&self, path: &str) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .get(self.config.url(path))
                .query(&self.config.query())
                .headers(self.config.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Make a GET request to {path} with given Query and deserialize the response body
    pub(crate) async fn get_with_query<Q, O>(&self, path: &str, query: &Q) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
        Q: Serialize + ?Sized,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .get(self.config.url(path))
                .query(&self.config.query())
                .query(query)
                .headers(self.config.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Make a DELETE request to {path} and deserialize the response body
    pub(crate) async fn delete<O>(&self, path: &str) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .delete(self.config.url(path))
                .query(&self.config.query())
                .headers(self.config.headers())
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// Make a POST request to {path} and return the response body
    pub(crate) async fn post_raw<I>(&self, path: &str, request: I) -> Result<Bytes, OpenAIError>
    where
        I: Serialize,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .post(self.config.url(path))
                .query(&self.config.query())
                .headers(self.config.headers())
                .json(&request)
                .build()?)
        };

        self.execute_raw(request_maker).await
    }

    /// Make a POST request to {path} and deserialize the response body
    pub(crate) async fn post<I, O>(&self, path: &str, request: I) -> Result<O, OpenAIError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .post(self.config.url(path))
                .query(&self.config.query())
                .headers(self.config.headers())
                .json(&request)
                .build()?)
        };

        self.execute(request_maker).await
    }

    /// POST a form at {path} and return the response body
    pub(crate) async fn post_form_raw<F>(&self, path: &str, form: F) -> Result<Bytes, OpenAIError>
    where
        reqwest::multipart::Form: async_convert::TryFrom<F, Error = OpenAIError>,
        F: Clone,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .post(self.config.url(path))
                .query(&self.config.query())
                .headers(self.config.headers())
                .multipart(async_convert::TryFrom::try_from(form.clone()).await?)
                .build()?)
        };

        self.execute_raw(request_maker).await
    }

    /// POST a form at {path} and deserialize the response body
    pub(crate) async fn post_form<O, F>(&self, path: &str, form: F) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
        reqwest::multipart::Form: async_convert::TryFrom<F, Error = OpenAIError>,
        F: Clone,
    {
        let request_maker = || async {
            Ok(self
                .http_client
                .post(self.config.url(path))
                .query(&self.config.query())
                .headers(self.config.headers())
                .multipart(async_convert::TryFrom::try_from(form.clone()).await?)
                .build()?)
        };

        self.execute(request_maker).await
    }

    // #[cfg(feature = "backoff")]
    // /// Execute a HTTP request and retry on rate limit
    // ///
    // /// request_maker serves one purpose: to be able to create request again
    // /// to retry API call after getting rate limited. request_maker is async because
    // /// reqwest::multipart::Form is created by async calls to read files for uploads.
    // async fn execute_raw<M, Fut>(&self, request_maker: M) -> Result<Bytes, OpenAIError>
    // where
    //     M: Fn() -> Fut,
    //     Fut: core::future::Future<Output = Result<reqwest::Request, OpenAIError>>,
    // {
    //     let client = self.http_client.clone();
    //
    //     backoff::future::retry(self.backoff.clone(), || async {
    //         let request = request_maker().await.map_err(backoff::Error::Permanent)?;
    //         let response = client
    //             .execute(request)
    //             .await
    //             .map_err(OpenAIError::Reqwest)
    //             .map_err(backoff::Error::Permanent)?;
    //
    //         let status = response.status();
    //         let bytes = response
    //             .bytes()
    //             .await
    //             .map_err(OpenAIError::Reqwest)
    //             .map_err(backoff::Error::Permanent)?;
    //
    //         // Deserialize response body from either error object or actual response object
    //         if !status.is_success() {
    //             let wrapped_error: WrappedError = serde_json::from_slice(bytes.as_ref())
    //                 .map_err(|e| map_deserialization_error(e, bytes.as_ref()))
    //                 .map_err(backoff::Error::Permanent)?;
    //
    //             if status.as_u16() == 429
    //                 // API returns 429 also when:
    //                 // "You exceeded your current quota, please check your plan and billing details."
    //                 && wrapped_error.error.r#type != Some("insufficient_quota".to_string())
    //             {
    //                 // Rate limited retry...
    //                 tracing::warn!("Rate limited: {}", wrapped_error.error.message);
    //                 return Err(backoff::Error::Transient {
    //                     err: OpenAIError::ApiError(wrapped_error.error),
    //                     retry_after: None,
    //                 });
    //             } else {
    //                 return Err(backoff::Error::Permanent(OpenAIError::ApiError(
    //                     wrapped_error.error,
    //                 )));
    //             }
    //         }
    //
    //         Ok(bytes)
    //     })
    //     .await
    // }

    #[cfg(not(feature = "backoff"))]
    /// Execute a HTTP request and retry on rate limit
    ///
    /// request_maker serves one purpose: to be able to create request again
    /// to retry API call after getting rate limited. request_maker is async because
    /// reqwest::multipart::Form is created by async calls to read files for uploads.
    async fn execute_raw<M, Fut>(&self, request_maker: M) -> Result<Bytes, OpenAIError>
        where
            M: Fn() -> Fut,
            Fut: future::Future<Output = Result<reqwest::Request, OpenAIError>>,
    {
        let client = self.http_client.clone();

        let request = request_maker().await?;
        let response = client
            .execute(request)
            .await
            .map_err(OpenAIError::Reqwest)?;

        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .map_err(OpenAIError::Reqwest)?;

        // Deserialize response body from either error object or actual response object
        if !status.is_success() {
            let wrapped_error: WrappedError = serde_json::from_slice(bytes.as_ref())
                .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

            if status.as_u16() == 429
                // API returns 429 also when:
                // "You exceeded your current quota, please check your plan and billing details."
                && wrapped_error.error.r#type != Some("insufficient_quota".to_string())
            {
                // Rate limited retry...
                tracing::warn!("Rate limited: {}", wrapped_error.error.message);
                return Err(OpenAIError::ApiError(wrapped_error.error));
            } else {
                return Err(OpenAIError::ApiError(wrapped_error.error));
            }
        }

        Ok(bytes)
    }

    /// Execute a HTTP request and retry on rate limit
    ///
    /// request_maker serves one purpose: to be able to create request again
    /// to retry API call after getting rate limited. request_maker is async because
    /// reqwest::multipart::Form is created by async calls to read files for uploads.
    async fn execute<O, M, Fut>(&self, request_maker: M) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
        M: Fn() -> Fut,
        Fut: core::future::Future<Output = Result<reqwest::Request, OpenAIError>>,
    {
        let bytes = self.execute_raw(request_maker).await?;

        let response: O = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        Ok(response)
    }

    /// Make HTTP POST request to receive SSE
    pub(crate) async fn post_stream<I, O>(
        &self,
        path: &str,
        request: I,
    ) -> OpenAIEventStream<O>
    where
        I: Serialize,
        O: DeserializeOwned + Send + 'static,
    {
        let event_source = self
            .http_client
            .post(self.config.url(path))
            .query(&self.config.query())
            .headers(self.config.headers())
            .json(&request)
            .eventsource()
            .unwrap();

        OpenAIEventStream::new(event_source)
    }

    /// Make HTTP GET request to receive SSE
    pub(crate) async fn get_stream<Q, O>(
        &self,
        path: &str,
        query: &Q,
    ) -> OpenAIEventStream<O>
    where
        Q: Serialize + ?Sized,
        O: DeserializeOwned + Send + 'static,
    {
        let event_source = self
            .http_client
            .get(self.config.url(path))
            .query(query)
            .query(&self.config.query())
            .headers(self.config.headers())
            .eventsource()
            .unwrap();

        OpenAIEventStream::new(event_source)
    }
}

pin_project! {
    /// Request which responds with SSE.
    /// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)
    pub struct OpenAIEventStream<O> {
        #[pin]
        stream: Filter<EventSource, future::Ready<bool>, fn(&Result<Event, reqwest_eventsource::Error>) -> future::Ready<bool>>,
        _phantom_data: PhantomData<O>
    }
}

impl<O> OpenAIEventStream<O> {
    pub(crate) fn new(event_source: EventSource) -> Self {
        Self {
            stream: event_source.filter(|result|
                // filter out the first event which is always Event::Open
                future::ready(!(result.is_ok()&&result.as_ref().unwrap().eq(&Event::Open)))
            ),
            _phantom_data: PhantomData
        }
    }
}

impl<O: DeserializeOwned + Send + 'static> Stream for OpenAIEventStream<O> {
    type Item = Result<O, OpenAIError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let stream: Pin<&mut _> = this.stream;
        match stream.poll_next(cx) {
            Poll::Ready(response) => {
                match response {
                    None => Poll::Ready(None), // end of the stream
                    Some(result) => match result {
                        Ok(event) => match event {
                            Event::Open => unreachable!(), // it has been filtered out
                            Event::Message(message) => {
                                if message.data == "[DONE]" {
                                    Poll::Ready(None)  // end of the stream, defined by OpenAI
                                } else {
                                    // deserialize the data
                                    match serde_json::from_str::<O>(&message.data) {
                                        Err(e) => Poll::Ready(Some(Err(map_deserialization_error(e, &message.data.as_bytes())))),
                                        Ok(output) => Poll::Ready(Some(Ok(output))),
                                    }
                                }
                            }
                        }
                        Err(e) => Poll::Ready(Some(Err(OpenAIError::StreamError(e.to_string()))))
                    }
                }
            }
            Poll::Pending => Poll::Pending
        }
    }
}
