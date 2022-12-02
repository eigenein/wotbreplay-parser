use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("failed to open the ZIP-archive")]
    OpenArchiveFailed(#[source] zip::result::ZipError),

    #[error("failed to open the battle results entry in the replay archive")]
    OpenBattleResultsFailed(#[source] zip::result::ZipError),

    #[error("failed to read the archive entry")]
    ReadEntryFailed(#[source] std::io::Error),

    #[error("failed to decode")]
    DecodeFailed(#[source] prost::DecodeError),

    #[error("failed to un-pickle")]
    UnpickleFailed(#[source] serde_pickle::Error),
}
