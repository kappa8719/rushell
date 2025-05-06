use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("could not do an async io job")]
    AsyncIO(#[from] std::io::Error),
    #[error("stream has ended already")]
    Disconnect,
    #[error("could not convert utf8 buffer to string")]
    FromUtf8(#[from] FromUtf8Error),
}
