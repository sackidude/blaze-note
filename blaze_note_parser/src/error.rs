//! # Error module
//!
//! This is the publicly facing API error. It represents all the ways parsing
//! of the custom markdown can fail.

use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnclosedBrackets,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnclosedBrackets => write!(f, "unclosed brackets at end of document"),
        }
    }
}

impl std::error::Error for Error {}
