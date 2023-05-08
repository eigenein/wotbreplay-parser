use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("ZIP error")]
    ZipError(#[from] zip::result::ZipError),

    #[error("Protocol Buffers error")]
    ProtobufError(#[from] prost::DecodeError),

    #[error("Pickle error")]
    PickleError(#[from] serde_pickle::Error),

    #[error("I/O error")]
    IoError(#[from] std::io::Error),

    #[error("UTF-8 error")]
    StringDecodeError(#[from] std::string::FromUtf8Error),

    #[cfg(feature = "meta")]
    #[error("JSON error")]
    JsonError(#[from] serde_json::Error),

    #[error("invalid magic: {actual:#X}, expected: {expected:#X}")]
    InvalidMagic { expected: u32, actual: u32 },

    #[error("failed to parse type {type_} packet's payload at replay time {clock_secs}s")]
    PacketPayloadParsingError {
        source: Box<Error>,
        type_: u32,
        clock_secs: f32,
    },
}
