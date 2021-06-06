use std::error::Error as StdError;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::ops::Deref;
use std::result::Result as StdResult;


#[derive(Debug)]
pub struct Error {
    description: String,
    cause: Option<Box<dyn StdError>>,
}


impl Error {
    pub fn new(description: String, cause: Option<Box<dyn StdError>>) -> Error {
        Error {
            description,
            cause,
        }
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> StdResult<(), FmtError> {
        write!(f, "{}", self.description)?;
        if let Some(ref cause) = self.cause {
            write!(f, ", caused by: {}", cause)?;
        }
        Ok(())
    }
}


impl StdError for Error {
    fn description(&self) -> &str {
        &self.description
    }

    fn cause(&self) -> Option<&dyn StdError> {
        match self.cause {
            Some(ref err) => Some(err.deref()),
            None => None,
        }
    }
}


impl From<&'static str> for Error {
    fn from(description: &'static str) -> Error {
        Error::new(description.to_string(), None)
    }
}


impl From<String> for Error {
    fn from(description: String) -> Error {
        Error::new(description, None)
    }
}


macro_rules! from_other_error {
    ($type:path) => {
        impl From<$type> for Error {
            fn from(cause: $type) -> Error {
                Error::new("An error occurred".to_owned(), Some(Box::new(cause)))
            }
        }
    };
}


from_other_error!(::std::io::Error);
from_other_error!(::image::ImageError);
from_other_error!(::serialport::Error);
from_other_error!(::sqlite::Error);
from_other_error!(::toml::de::Error);


pub type Result<T> = StdResult<T, Error>;
