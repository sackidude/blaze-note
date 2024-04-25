//! # Error module
//!
//! This is the publicly facing API error. It represents all the ways parsing
//! of the custom markdown can fail.

use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidNoteSyntax,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidNoteSyntax => write!(f, "invalid note syntax"),
        }
    }
}

impl std::error::Error for Error {}
