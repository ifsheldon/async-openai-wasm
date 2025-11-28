use std::fmt::Display;

use crate::types::images::{
    DallE2ImageSize, ImageBackground, ImageEditInput, ImageInput, ImageModel, ImageOutputFormat,
    ImageQuality, ImageResponseFormat, ImageSize, InputFidelity,
};

impl Display for ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::S256x256 => "256x256",
                Self::S512x512 => "512x512",
                Self::S1024x1024 => "1024x1024",
                Self::S1792x1024 => "1792x1024",
                Self::S1024x1792 => "1024x1792",
                Self::S1536x1024 => "1536x1024",
                Self::S1024x1536 => "1024x1536",
                Self::Auto => "auto",
            }
        )
    }
}

impl Display for DallE2ImageSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::S256x256 => "256x256",
                Self::S512x512 => "512x512",
                Self::S1024x1024 => "1024x1024",
            }
        )
    }
}

impl Display for ImageModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DallE2 => "dall-e-2",
                Self::DallE3 => "dall-e-3",
                Self::GptImage1 => "gpt-image-1",
                Self::GptImage1Mini => "gpt-image-1-mini",
                Self::Other(other) => other,
            }
        )
    }
}

impl Display for ImageBackground {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Transparent => "transparent",
                Self::Opaque => "opaque",
                Self::Auto => "auto",
            }
        )
    }
}

impl Display for ImageOutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Png => "png",
                Self::Jpeg => "jpeg",
                Self::Webp => "webp",
            }
        )
    }
}

impl Display for InputFidelity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::High => "high",
                Self::Low => "low",
            }
        )
    }
}

impl Display for ImageQuality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Low => "low",
                Self::Medium => "medium",
                Self::High => "high",
                Self::Auto => "auto",
                Self::Standard => "standard",
                Self::HD => "hd",
            }
        )
    }
}

impl Display for ImageResponseFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Url => "url",
                Self::B64Json => "b64_json",
            }
        )
    }
}

impl Default for ImageEditInput {
    fn default() -> Self {
        Self::Image(ImageInput::default())
    }
}

impl From<ImageInput> for ImageEditInput {
    fn from(value: ImageInput) -> Self {
        Self::Image(value)
    }
}

impl From<Vec<ImageInput>> for ImageEditInput {
    fn from(value: Vec<ImageInput>) -> Self {
        Self::Images(value)
    }
}
