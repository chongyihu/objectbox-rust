use crate::c;
use std::{error, fmt, result};

#[derive(Clone)]
enum Repr {
    Native(c::NativeError),
    Local(String),
}

#[derive(Clone)]
pub struct Error {
    repr: Repr,
}

impl Error {
    pub fn new_native(native_error: c::NativeError) -> Error {
        Error {
            repr: Repr::Native(native_error),
        }
    }

    pub fn new_local(local_error: &str) -> Error {
        Error {
            repr: Repr::Local(String::from(local_error)),
        }
    }

    pub fn as_result<T>(&self) -> Result<T> {
        Err(self.clone())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.repr {
            Repr::Native(ref err) => write!(fmt, "{}", err),
            Repr::Local(s) => write!(fmt, "{}", s),
        }
    }
}

impl fmt::Debug for Repr {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &*self {
            Repr::Native(ref err) => fmt::Debug::fmt(&err, fmt),
            Repr::Local(s) => fmt::Debug::fmt(&s, fmt),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.repr, f)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self.repr {
            Repr::Native(ref err) => err.source(),
            Repr::Local(_) => None,
        }
    }
}

// A specialized result
pub type Result<T> = result::Result<T, Error>;

#[cfg(test)]
mod tests {
    use super::*;

    /// verify the installed library version is the same as the version from objectbox.h
    #[test]
    fn fail_local_error() {
        let err = Error::new_local("test");
        assert_eq!(format!("{err}"), "test");
    }
}
