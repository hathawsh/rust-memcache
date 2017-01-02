use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum MemcacheError {
    Io(io::Error),
    Error,
    ClientError(String),
    ServerError(String),
}

impl fmt::Display for MemcacheError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MemcacheError::Io(ref err) => err.fmt(f),
            MemcacheError::Error => write!(f, "Error"),
            MemcacheError::ClientError(ref s) => s.fmt(f),
            MemcacheError::ServerError(ref s) => s.fmt(f),
        }
    }
}

impl error::Error for MemcacheError {
    fn description(&self) -> &str {
        match *self {
            MemcacheError::Io(ref err) => err.description(),
            MemcacheError::Error => "Error",
            MemcacheError::ClientError(ref s) => s.as_str(),
            MemcacheError::ServerError(ref s) => s.as_str(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            MemcacheError::Io(ref err) => Some(err),
            MemcacheError::Error => None,
            MemcacheError::ClientError(_) => None,
            MemcacheError::ServerError(_) => None,
        }
    }
}

impl From<io::Error> for MemcacheError {
    fn from(err: io::Error) -> MemcacheError {
        MemcacheError::Io(err)
    }
}

impl From<String> for MemcacheError {
    fn from(s: String) -> MemcacheError {
        if s == "ERROR\r\n" {
            return MemcacheError::Error;
        }  else if s.starts_with("CLIENT_ERROR") {
            return MemcacheError::ClientError(s);
        } else if s.starts_with("SERVER_ERROR") {
            return MemcacheError::ServerError(s);
        } else {
            panic!("{} if not a memcached error!", s);
        }
    }
}

pub fn is_memcache_error(s: &str) -> bool {
    return s == "ERROR\r\n" || s.starts_with("CIENT_ERROR") || s.starts_with("SERVER_ERROR");
}
