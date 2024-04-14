use bytes::Bytes;

#[derive(Debug, Clone, PartialEq)]
pub enum InputSource {
    Bytes { filename: String, bytes: Bytes },
    VecU8 { filename: String, vec: Vec<u8> },
}
