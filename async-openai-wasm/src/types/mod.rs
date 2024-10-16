//! Types used in OpenAI API requests and responses.
//! These types are created from component schemas in the [OpenAPI spec](https://github.com/openai/openai-openapi)
use derive_builder::UninitializedFieldError;

pub use assistant::*;
pub use assistant_file::*;
pub use assistant_stream::*;
pub use audio::*;
pub use batch::*;
pub use chat::*;
pub use common::*;
pub use completion::*;
pub use embedding::*;
pub use file::*;
pub use fine_tuning::*;
pub use image::*;
pub use message::*;
pub use message_file::*;
pub use model::*;
pub use moderation::*;
pub use run::*;
pub use step::*;
pub use thread::*;
pub use vector_store::*;

use crate::error::OpenAIError;

mod assistant;
mod assistant_file;
mod assistant_impls;
mod assistant_stream;
mod audio;
mod batch;
mod chat;
mod common;
mod completion;
mod embedding;
mod file;
mod fine_tuning;
mod image;
mod message;
mod message_file;
mod model;
mod moderation;
#[cfg_attr(docsrs, doc(cfg(feature = "realtime")))]
#[cfg(feature = "realtime")]
pub mod realtime;
mod run;
mod step;
mod thread;
mod vector_store;

mod impls;

impl From<UninitializedFieldError> for OpenAIError {
    fn from(value: UninitializedFieldError) -> Self {
        OpenAIError::InvalidArgument(value.to_string())
    }
}
