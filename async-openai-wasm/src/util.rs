use reqwest::Body;

use crate::error::OpenAIError;
use crate::types::InputSource;

/// Creates the part for the given file for multipart upload.
pub(crate) async fn create_file_part(
    source: InputSource,
) -> Result<reqwest::multipart::Part, OpenAIError> {
    let (stream, file_name) = match source {
        InputSource::Bytes { filename, bytes } => {
            (Body::from(bytes), filename)
        }
        InputSource::VecU8 { filename, vec } => {
            (Body::from(vec), filename)
        }
    };

    let file_part = reqwest::multipart::Part::stream(stream)
        .file_name(file_name)
        .mime_str("application/octet-stream")
        .unwrap();

    Ok(file_part)
}

