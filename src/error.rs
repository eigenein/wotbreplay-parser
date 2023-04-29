use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ZIP error")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Protocol Buffers error")]
    DecodeFailed(#[from] prost::DecodeError),

    #[error("Pickle error")]
    UnpickleFailed(#[from] serde_pickle::Error),

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("UTF-8 error")]
    StringDecodeError(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "meta")]
    #[error("JSON error")]
    JsonDecodeFailed(#[from] serde_json::Error),

    #[error("invalid magic: {0:#X}, expected: {1:#X}")]
    InvalidMagic(u32, u32),
}
