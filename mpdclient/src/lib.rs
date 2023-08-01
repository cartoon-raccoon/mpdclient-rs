use std::io::Error as IoError;

pub mod player;

use player::Player;

mod raw;

use raw::RawMpdClient;


pub struct MpdClient {
    inner: RawMpdClient,
}

impl MpdClient {
    pub fn new<S: AsRef<str>>(host: S, port: u16, timeout: u32) -> Result<Self> {
        Ok(Self {
            inner: RawMpdClient::new(host.as_ref(), port, timeout)?
        })
    }

    pub fn default() -> Result<Self> {
        Ok(Self {
            inner: RawMpdClient::new_default()?
        })
    }

    pub fn player(&mut self) -> Player<'_> {
        Player {
            inner: &mut self.inner
        }
    }
}

pub type Result<T> = ::core::result::Result<T, MpdError>;

#[derive(Debug)]
pub struct MpdError {
    pub(crate) kind: MpdErrorKind,
    pub(crate) errmsg: String,
}

impl MpdError {
    pub fn kind(&self) -> &MpdErrorKind {
        &self.kind
    }

    pub fn errormsg(&self) -> &str {
        &self.errmsg
    }

}

#[repr(i32)]
#[derive(Debug, Clone, Copy)]
pub enum ServerError {
    Unknown = -1,
    NotListed = 1,
    Arguments = 2,
    Password = 3,
    Permissions = 4,
    UnknownCmd = 5,
    NoExist = 50,
    PlaylistMax = 51,
    System = 52,
    PlaylistLoad = 53,
    UpdateAlready = 54,
    PlayerSync = 55,
    Exists = 56,
}

#[derive(Debug)]
pub enum MpdErrorKind {
    Io(IoError),
    Arguments,
    UnknownHost,
    Malformed,
    Server(ServerError),
}