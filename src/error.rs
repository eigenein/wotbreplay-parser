use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to open the replay as a ZIP-archive")]
    OpenArchiveFailed(#[source] zip::result::ZipError),

    #[error("failed to open the entry in the replay archive")]
    OpenEntryFailed(#[source] zip::result::ZipError),

    #[error("failed to decode the Protocol Buffers encoded data")]
    DecodeFailed(#[source] prost::DecodeError),

    #[error("failed to un-pickle")]
    UnpickleFailed(#[source] serde_pickle::Error),

    #[cfg(feature = "meta")]
    #[error("failed to decode JSON")]
    JsonDecodeFailed(#[source] serde_json::Error),
}
