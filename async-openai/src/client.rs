use std::future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use bytes::Bytes;
use eventsource_stream::EventStreamError;
use future::Future;
use futures::stream::Filter;
use futures::{Stream, stream::StreamExt};
use pin_project::pin_project;
use reqwest::header::HeaderMap;
use reqwest::{Response, multipart::Form};
use reqwest_eventsource::{Event, EventSource, RequestBuilderExt};
use serde::{Serialize, de::DeserializeOwned};

use crate::error::{ApiError, StreamError};
use crate::{
    config::{Config, OpenAIConfig},
    error::{OpenAIError, WrappedError, map_deserialization_error},
    traits::AsyncTryFrom,
    RequestOptions,
};

#[cfg(feature = "administration")]
use crate::admin::Admin;
#[cfg(feature = "chatkit")]
use crate::chatkit::Chatkit;
#[cfg(feature = "file")]
use crate::file::Files;
#[cfg(feature = "image")]
use crate::image::Images;
#[cfg(feature = "moderation")]
use crate::moderation::Moderations;
#[cfg(feature = "assistant")]
use crate::Assistants;
#[cfg(feature = "audio")]
use crate::Audio;
#[cfg(feature = "batch")]
use crate::Batches;
#[cfg(feature = "chat-completion")]
use crate::Chat;
#[cfg(feature = "completions")]
use crate::Completions;
#[cfg(feature = "container")]
use crate::Containers;
#[cfg(feature = "responses")]
use crate::Conversations;
#[cfg(feature = "embedding")]
use crate::Embeddings;
#[cfg(feature = "evals")]
use crate::Evals;
#[cfg(feature = "finetuning")]
use crate::FineTuning;
#[cfg(feature = "model")]
use crate::Models;
#[cfg(feature = "realtime")]
use crate::Realtime;
#[cfg(feature = "responses")]
use crate::Responses;
#[cfg(feature = "assistant")]
use crate::Threads;
#[cfg(feature = "upload")]
use crate::Uploads;
#[cfg(feature = "vectorstore")]
use crate::VectorStores;
#[cfg(feature = "video")]
use crate::Videos;

#[derive(Debug, Clone)]
/// Client is a container for config and http_client
/// used to make API calls.
pub struct Client<C: Config> {
    http_client: reqwest::Client,
    config: C,
}

impl Client<OpenAIConfig> {
    /// Client with default [OpenAIConfig]
    pub fn new() -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config: OpenAIConfig::default(),
        }
    }
}

impl<C: Config> Client<C> {
    /// Create client with a custom HTTP client, OpenAI config
    pub fn build(http_client: reqwest::Client, config: C) -> Self {
        Self {
            http_client,
            config,
        }
    }

    /// Create client with [OpenAIConfig] or [crate::config::AzureConfig]
    pub fn with_config(config: C) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            config,
        }
    }

    /// Provide your own [client] to make HTTP requests with.
    ///
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }

    // API groups

    /// To call [Models] group related APIs using this client.
    #[cfg(feature = "model")]
    pub fn models(&self) -> Models<'_, C> {
        Models::new(self)
    }

    /// To call [Completions] group related APIs using this client.
    #[cfg(feature = "completions")]
    pub fn completions(&self) -> Completions<'_, C> {
        Completions::new(self)
    }

    /// To call [Chat] group related APIs using this client.
    #[cfg(feature = "chat-completion")]
    pub fn chat(&self) -> Chat<'_, C> {
        Chat::new(self)
    }

    /// To call [Images] group related APIs using this client.
    #[cfg(feature = "image")]
    pub fn images(&self) -> Images<'_, C> {
        Images::new(self)
    }

    /// To call [Moderations] group related APIs using this client.
    #[cfg(feature = "moderation")]
    pub fn moderations(&self) -> Moderations<'_, C> {
        Moderations::new(self)
    }

    /// To call [Files] group related APIs using this client.
    #[cfg(feature = "file")]
    pub fn files(&self) -> Files<'_, C> {
        Files::new(self)
    }

    /// To call [Uploads] group related APIs using this client.
    #[cfg(feature = "upload")]
    pub fn uploads(&self) -> Uploads<'_, C> {
        Uploads::new(self)
    }

    /// To call [FineTuning] group related APIs using this client.
    #[cfg(feature = "finetuning")]
    pub fn fine_tuning(&self) -> FineTuning<'_, C> {
        FineTuning::new(self)
    }

    /// To call [Embeddings] group related APIs using this client.
    #[cfg(feature = "embedding")]
    pub fn embeddings(&self) -> Embeddings<'_, C> {
        Embeddings::new(self)
    }

    /// To call [Audio] group related APIs using this client.
    #[cfg(feature = "audio")]
    pub fn audio(&self) -> Audio<'_, C> {
        Audio::new(self)
    }

    /// To call [Videos] group related APIs using this client.
    #[cfg(feature = "video")]
    pub fn videos(&self) -> Videos<'_, C> {
        Videos::new(self)
    }

    /// To call [Assistants] group related APIs using this client.
    #[cfg(feature = "assistant")]
    pub fn assistants(&self) -> Assistants<'_, C> {
        Assistants::new(self)
    }

    /// To call [Threads] group related APIs using this client.
    #[cfg(feature = "assistant")]
    pub fn threads(&self) -> Threads<'_, C> {
        Threads::new(self)
    }

    /// To call [VectorStores] group related APIs using this client.
    #[cfg(feature = "vectorstore")]
    pub fn vector_stores(&self) -> VectorStores<'_, C> {
        VectorStores::new(self)
    }

    /// To call [Batches] group related APIs using this client.
    #[cfg(feature = "batch")]
    pub fn batches(&self) -> Batches<'_, C> {
        Batches::new(self)
    }

    /// To call [Admin] group related APIs using this client.
    /// This groups together admin API keys, invites, users, projects, audit logs, and certificates.
    #[cfg(feature = "administration")]
    pub fn admin(&self) -> Admin<'_, C> {
        Admin::new(self)
    }

    /// To call [Responses] group related APIs using this client.
    #[cfg(feature = "responses")]
    pub fn responses(&self) -> Responses<'_, C> {
        Responses::new(self)
    }

    /// To call [Conversations] group related APIs using this client.
    #[cfg(feature = "responses")]
    pub fn conversations(&self) -> Conversations<'_, C> {
        Conversations::new(self)
    }

    /// To call [Containers] group related APIs using this client.
    #[cfg(feature = "container")]
    pub fn containers(&self) -> Containers<'_, C> {
        Containers::new(self)
    }

    /// To call [Evals] group related APIs using this client.
    #[cfg(feature = "evals")]
    pub fn evals(&self) -> Evals<'_, C> {
        Evals::new(self)
    }

    #[cfg(feature = "chatkit")]
    pub fn chatkit(&self) -> Chatkit<'_, C> {
        Chatkit::new(self)
    }

    /// To call [Realtime] group related APIs using this client.
    #[cfg(feature = "realtime")]
    pub fn realtime(&self) -> Realtime<'_, C> {
        Realtime::new(self)
    }

    pub fn config(&self) -> &C {
        &self.config
    }

    /// Helper function to build a request builder with common configuration
    fn build_request_builder(
        &self,
        method: reqwest::Method,
        path: &str,
        request_options: &RequestOptions,
    ) -> reqwest::RequestBuilder {
        let mut request_builder = if let Some(path) = request_options.path() {
            self.http_client
                .request(method, self.config.url(path.as_str()))
        } else {
            self.http_client.request(method, self.config.url(path))
        };

        request_builder = request_builder
            .query(&self.config.query())
            .headers(self.config.headers());

        if let Some(headers) = request_options.headers() {
            request_builder = request_builder.headers(headers.clone());
        }

        if !request_options.query().is_empty() {
            request_builder = request_builder.query(request_options.query());
        }

        request_builder
    }

    /// Make a GET request to {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn get<O>(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        self.execute(async {
            Ok(self
                .build_request_builder(reqwest::Method::GET, path, request_options)
                .build()?)
        })
        .await
    }

    /// Make a DELETE request to {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn delete<O>(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        self.execute(async {
            Ok(self
                .build_request_builder(reqwest::Method::DELETE, path, request_options)
                .build()?)
        })
        .await
    }

    /// Make a GET request to {path} and return the response body
    #[allow(unused)]
    pub(crate) async fn get_raw(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> Result<(Bytes, HeaderMap), OpenAIError> {
        self.execute_raw(async {
            Ok(self
                .build_request_builder(reqwest::Method::GET, path, request_options)
                .build()?)
        })
        .await
    }

    /// Make a POST request to {path} and return the response body
    #[allow(unused)]
    pub(crate) async fn post_raw<I>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> Result<(Bytes, HeaderMap), OpenAIError>
    where
        I: Serialize,
    {
        self.execute_raw(async {
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .json(&request)
                .build()?)
        })
        .await
    }

    /// Make a POST request to {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn post<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        I: Serialize,
        O: DeserializeOwned,
    {
        self.execute(async {
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .json(&request)
                .build()?)
        })
        .await
    }

    /// POST a form at {path} and return the response body
    #[allow(unused)]
    pub(crate) async fn post_form_raw<F>(
        &self,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<(Bytes, HeaderMap), OpenAIError>
    where
        Form: AsyncTryFrom<F, Error = OpenAIError>,
    {
        self.execute_raw(async {
            let form = <Form as AsyncTryFrom<F>>::try_from(form).await?;
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .multipart(form)
                .build()?)
        })
        .await
    }

    /// POST a form at {path} and deserialize the response body
    #[allow(unused)]
    pub(crate) async fn post_form<O, F>(
        &self,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
    {
        self.execute(async {
            let form = <Form as AsyncTryFrom<F>>::try_from(form).await?;
            Ok(self
                .build_request_builder(reqwest::Method::POST, path, request_options)
                .multipart(form)
                .build()?)
        })
        .await
    }

    #[allow(unused)]
    pub(crate) async fn post_form_stream<O, F>(
        &self,
        path: &str,
        form: F,
        request_options: &RequestOptions,
    ) -> Result<OpenAIFormEventStream<O>, OpenAIError>
    where
        F: Clone,
        Form: AsyncTryFrom<F, Error = OpenAIError>,
        O: DeserializeOwned + Send + 'static,
    {
        // Build and execute request manually since multipart::Form is not Clone
        // and .eventsource() requires cloneability
        let request_builder = self
            .build_request_builder(reqwest::Method::POST, path, request_options)
            .multipart(<Form as AsyncTryFrom<F>>::try_from(form.clone()).await?);

        let response = request_builder.send().await.map_err(OpenAIError::Reqwest)?;

        // Check for error status
        if !response.status().is_success() {
            return Err(read_response(response).await.unwrap_err());
        }

        // Convert response body to EventSource stream
        let stream = response
            .bytes_stream()
            .map(|result| result.map_err(std::io::Error::other));
        let event_stream = eventsource_stream::EventStream::new(stream);

        Ok(OpenAIFormEventStream::new(event_stream))
    }

    /// Execute a HTTP request
    async fn execute_raw(
        &self,
        request_future: impl Future<Output = Result<reqwest::Request, OpenAIError>>,
    ) -> Result<(Bytes, HeaderMap), OpenAIError> {
        let client = self.http_client.clone();
        let request = request_future.await?;
        let response = client
            .execute(request)
            .await
            .map_err(OpenAIError::Reqwest)?;

        let status = response.status();
        match read_response(response).await {
            Ok((bytes, headers)) => Ok((bytes, headers)),
            Err(e) => match e {
                OpenAIError::ApiError(api_error) => {
                    if status.as_u16() == 429
                        && api_error.r#type != Some("insufficient_quota".to_string())
                    {
                        // Rate limited retry...
                        tracing::warn!("Rate limited: {}", api_error.message);
                    }
                    Err(OpenAIError::ApiError(api_error))
                }
                _ => Err(e),
            },
        }
    }

    /// Execute a HTTP request
    async fn execute<O>(
        &self,
        request_future: impl Future<Output = Result<reqwest::Request, OpenAIError>>,
    ) -> Result<O, OpenAIError>
    where
        O: DeserializeOwned,
    {
        let (bytes, _headers) = self.execute_raw(request_future).await?;

        let response: O = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        Ok(response)
    }

    /// Make HTTP POST request to receive SSE
    #[allow(unused)]
    pub(crate) async fn post_stream<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
    ) -> OpenAIEventStream<O>
    where
        I: Serialize,
        O: DeserializeOwned + Send + 'static,
    {
        let request_builder = self
            .build_request_builder(reqwest::Method::POST, path, request_options)
            .json(&request);

        let event_source = request_builder.eventsource().unwrap();

        OpenAIEventStream::new(event_source)
    }

    #[allow(unused)]
    pub(crate) async fn post_stream_mapped_raw_events<I, O>(
        &self,
        path: &str,
        request: I,
        request_options: &RequestOptions,
        event_mapper: impl Fn(eventsource_stream::Event) -> Result<O, OpenAIError> + Send + 'static,
    ) -> OpenAIEventStream<O>
    where
        I: Serialize,
        O: DeserializeOwned + Send + 'static,
    {
        let request_builder = self
            .build_request_builder(reqwest::Method::POST, path, request_options)
            .json(&request);

        let event_source = request_builder.eventsource().unwrap();

        OpenAIEventStream::with_event_mapping(event_source, event_mapper)
    }

    /// Make HTTP GET request to receive SSE
    #[allow(unused)]
    pub(crate) async fn get_stream<O>(
        &self,
        path: &str,
        request_options: &RequestOptions,
    ) -> OpenAIEventStream<O>
    where
        O: DeserializeOwned + Send + 'static,
    {
        let request_builder =
            self.build_request_builder(reqwest::Method::GET, path, request_options);

        let event_source = request_builder.eventsource().unwrap();

        OpenAIEventStream::new(event_source)
    }
}

// TODO: check the implementation of OpenAIEventStream to reflect new changes from #485

/// Request which responds with SSE.
/// [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#event_stream_format)

#[pin_project]
pub struct OpenAIEventStream<O>
where
    O: DeserializeOwned + Send + 'static,
{
    #[pin]
    stream: Filter<
        EventSource,
        future::Ready<bool>,
        fn(&Result<Event, reqwest_eventsource::Error>) -> future::Ready<bool>,
    >,
    event_mapper:
        Option<Box<dyn Fn(eventsource_stream::Event) -> Result<O, OpenAIError> + Send + 'static>>,
    done: bool,
    _phantom_data: PhantomData<O>,
}

impl<O> OpenAIEventStream<O>
where
    O: DeserializeOwned + Send + 'static,
{
    pub(crate) fn with_event_mapping<M>(event_source: EventSource, event_mapper: M) -> Self
    where
        M: Fn(eventsource_stream::Event) -> Result<O, OpenAIError> + Send + 'static,
    {
        Self {
            stream: event_source.filter(|result|
                // filter out the first event which is always Event::Open
                future::ready(!(result.is_ok() && result.as_ref().unwrap().eq(&Event::Open)))),
            done: false,
            event_mapper: Some(Box::new(event_mapper)),
            _phantom_data: PhantomData,
        }
    }

    pub(crate) fn new(event_source: EventSource) -> Self {
        Self {
            stream: event_source.filter(|result|
                // filter out the first event which is always Event::Open
                future::ready(!(result.is_ok() && result.as_ref().unwrap().eq(&Event::Open)))),
            done: false,
            event_mapper: None,
            _phantom_data: PhantomData,
        }
    }
}

impl<O> Stream for OpenAIEventStream<O>
where
    O: DeserializeOwned + Send + 'static,
{
    type Item = Result<O, OpenAIError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if *this.done {
            return Poll::Ready(None);
        }
        let stream: Pin<&mut _> = this.stream;
        match stream.poll_next(cx) {
            Poll::Ready(response) => {
                match response {
                    None => Poll::Ready(None), // end of the stream
                    Some(result) => match result {
                        Ok(event) => match event {
                            Event::Open => unreachable!(), // it has been filtered out
                            Event::Message(message) => {
                                if let Some(event_mapper) = this.event_mapper.as_ref() {
                                    if message.data == "[DONE]" {
                                        *this.done = true;
                                    }
                                    let response = event_mapper(message);
                                    match response {
                                        Ok(output) => Poll::Ready(Some(Ok(output))),
                                        Err(_) => Poll::Ready(None),
                                    }
                                } else {
                                    if message.data == "[DONE]" {
                                        *this.done = true;
                                        Poll::Ready(None) // end of the stream, defined by OpenAI
                                    } else {
                                        // deserialize the data
                                        match serde_json::from_str::<O>(&message.data) {
                                            Err(e) => {
                                                *this.done = true;
                                                Poll::Ready(Some(Err(map_deserialization_error(
                                                    e,
                                                    message.data.as_bytes(),
                                                ))))
                                            }
                                            Ok(output) => Poll::Ready(Some(Ok(output))),
                                        }
                                    }
                                }
                            }
                        },
                        Err(e) => {
                            *this.done = true;
                            Poll::Ready(Some(Err(OpenAIError::StreamError(Box::new(
                                StreamError::ReqwestEventSource(e),
                            )))))
                        }
                    },
                }
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

#[pin_project]
pub struct OpenAIFormEventStream<O>
where
    O: DeserializeOwned + Send + 'static,
{
    #[pin]
    event_stream: Box<
        dyn Stream<Item = Result<eventsource_stream::Event, EventStreamError<std::io::Error>>>
            + Unpin
            + 'static,
    >,
    done: bool,
    _phantom_data: PhantomData<O>,
}

impl<O> OpenAIFormEventStream<O>
where
    O: DeserializeOwned + Send + 'static,
{
    pub fn new(
        stream: impl Stream<Item = Result<eventsource_stream::Event, EventStreamError<std::io::Error>>>
        + Unpin
        + 'static,
    ) -> Self {
        Self {
            event_stream: Box::new(stream),
            done: false,
            _phantom_data: PhantomData,
        }
    }
}

impl<O> Stream for OpenAIFormEventStream<O>
where
    O: DeserializeOwned + Send + 'static,
{
    type Item = Result<O, OpenAIError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.project();
        if *this.done {
            return Poll::Ready(None);
        }
        let stream: Pin<&mut _> = this.event_stream;
        match stream.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(response) => match response {
                None => Poll::Ready(None),
                Some(result) => match result {
                    Err(e) => {
                        // *this.done = true; // TODO: check this
                        Poll::Ready(Some(Err(OpenAIError::StreamError(Box::new(
                            StreamError::EventStream(e.to_string()),
                        )))))
                    }
                    Ok(event) => {
                        if event.data == "[DONE]" {
                            *this.done = true;
                            Poll::Ready(None)
                        } else {
                            match serde_json::from_str::<O>(&event.data) {
                                Err(e) => Poll::Ready(Some(Err(map_deserialization_error(
                                    e,
                                    event.data.as_bytes(),
                                )))),
                                Ok(output) => Poll::Ready(Some(Ok(output))),
                            }
                        }
                    }
                },
            },
        }
    }
}

async fn read_response(response: Response) -> Result<(Bytes, HeaderMap), OpenAIError> {
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = response.bytes().await.map_err(OpenAIError::Reqwest)?;

    if status.is_server_error() {
        // OpenAI does not guarantee server errors are returned as JSON so we cannot deserialize them.
        let message: String = String::from_utf8_lossy(&bytes).into_owned();
        tracing::warn!("Server error: {status} - {message}");
        return Err(OpenAIError::ApiError(ApiError {
            message,
            r#type: None,
            param: None,
            code: None,
        }));
    }

    // Deserialize response body from either error object or actual response object
    if !status.is_success() {
        let wrapped_error: WrappedError = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        return Err(OpenAIError::ApiError(wrapped_error.error));
    }

    Ok((bytes, headers))
}
