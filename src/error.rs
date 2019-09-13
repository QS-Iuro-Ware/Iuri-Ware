use log::{debug, error};
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
    fn from(err: serde_json::Error) -> Self {
        debug!("Serde Error: {}", err);
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
            Self::MustJoinRoom => write!(f, "Must join room first"),
            Self::JsonParsingFailed => write!(f, "Unable to parse json"),
            Self::NoRoom(room) => write!(f, "Failed to find room {}", room),
            Self::MailBox(_) | Self::AddrNotFound(_) => {
                error!("{:?}", self);
                write!(f, "Internal Server Error")
            }
        }
    }
}

impl Error for IuroError {}
