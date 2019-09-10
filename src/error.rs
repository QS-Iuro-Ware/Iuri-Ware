use std::{error::Error, fmt, fmt::Display, fmt::Formatter};

#[derive(Debug)]
pub enum IuroError {
    MustJoinRoom,
    NoRoom(String),
    AddrNotFound(usize),
    JsonParsingFailed,
    MailBox(actix::MailboxError),
}

impl From<serde_json::Error> for IuroError {
    fn from(_: serde_json::Error) -> Self {
        Self::JsonParsingFailed
    }
}

impl From<actix::MailboxError> for IuroError {
    fn from(m: actix::MailboxError) -> Self {
        Self::MailBox(m)
    }
}

impl Display for IuroError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            IuroError::MustJoinRoom => write!(f, "Must join room first"),
            IuroError::JsonParsingFailed => write!(f, "Unable to parse json"),
            IuroError::NoRoom(room) => write!(f, "Failed to find room {}", room),
            IuroError::MailBox(_) => write!(f, "Internal Server Error"),
            IuroError::AddrNotFound(_) => write!(f, "Internal Server Error"),
        }
    }
}

impl Error for IuroError {}
