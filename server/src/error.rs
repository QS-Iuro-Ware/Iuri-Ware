use log::error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IuroError {
    #[error("No game currently running")]
    NoGameRunning,
    #[error("Must join room first")]
    MustJoinRoom,
    #[error("Unable to parse message")]
    JsonParsingFailed(#[from] serde_json::Error),
    #[error("Room `{0}` not found")]
    NoRoom(String),
    #[error("Room `{0}` is full")]
    FullRoom(String),
    #[error("{}", internal_error(.0))]
    AddrNotFound(usize),
    #[error("{}", internal_error(.0))]
    MailBox(#[from] actix::MailboxError),
}

fn internal_error(error: impl std::fmt::Debug) -> &'static str {
    error!("{:?}", error);
    "Internal Server Error"
}
