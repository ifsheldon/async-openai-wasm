#[derive(Debug, Clone, PartialEq)]
pub enum InputSource {
    Bytes {
        filename: String,
        bytes: bytes::Bytes,
    },
    VecU8 {
        filename: String,
        vec: Vec<u8>,
    },
}

impl Default for InputSource {
    fn default() -> Self {
        InputSource::VecU8 {
            filename: String::default(),
            vec: Vec::new(),
        }
    }
}
