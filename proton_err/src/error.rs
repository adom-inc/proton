//! Enumeration of Proton AP management errors.

use std::{
    error::Error,
    fmt::{
        Display,
        Debug,
        Formatter,
        Result,
    },
};

#[derive(Debug)]
/// An error that occurred within the Proton library.
/// 
/// Note: this enumeration is exhaustive because of the `Other` variant.
pub enum ProtonError<T> 
    where T: Display + Debug
{
    /// An error that could not be converted to a native error.
    Other (T)
}

impl<T> Display for ProtonError<T>
    where T: Display + Debug
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use ProtonError::*;
        let error = match self {
            Other (t) => t.to_string(),
        };

        write!(f, "{}", error)
    }
}

impl<T> Error for ProtonError<T>
    where T: Display + Debug { }

impl From<Box<dyn Error>> for ProtonError<String> {
    fn from(e: Box<dyn Error>) -> ProtonError<String> {
        let string = if let Some (err) = e.source() {
            err.to_string()
        } else {
            String::new()
        };

        ProtonError::<String>::Other (string)
    }
}