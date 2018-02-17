use std::fmt;
use std::result::Result as StdResult;
use std::error::Error as StdError;
use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum Error {
  MySql(DieselError)
}

pub type Result<T> = StdResult<T, Error>;

impl fmt::Display for Error {
  fn fmt(&self, fm: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Error::MySql(ref err) => {
        fm.write_str("Error::Io ")?;
        fmt::Display::fmt(err, fm)
      }
    }
  }
}

impl StdError for Error {
  fn description(&self) -> &str {
    match *self {
      Error::MySql(ref err) => err.description(),
    }
  }

  fn cause(&self) -> Option<&StdError> {
    match *self {
      Error::MySql(ref err) => Some(err),
    }
  }
}

impl From<DieselError> for Error {
    fn from(err: DieselError) -> Self {
        Error::MySql(err)
    }
}
