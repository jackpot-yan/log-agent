use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("channel send error")]
    SendError,

    #[error("channel receive error")]
    RecvError,

    #[error("system time error: {0}")]
    SystemTime(#[from] std::time::SystemTimeError),
}

pub type Result<T> = std::result::Result<T, Error>;
