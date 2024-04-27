//! # Error module
//!
//! This is the publicly facing API error. It represents all the ways parsing
//! of the custom markdown can fail.

use core::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnclosedBrackets,
    EmptyCard,
    NoOpeningBrackets,
    MalformedBars,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::UnclosedBrackets => write!(f, "unclosed brackets at end of document"),
            Error::EmptyCard => write!(
                f,
                "empty card, without any of the card indicators '|' '||' '|>'"
            ),
            Error::NoOpeningBrackets => write!(
                f,
                "got closing brackets `}}` before any opening brackets `{{`"
            ),
            Error::MalformedBars => write!(
                f,
                "Malformed bars `|` somewhere, could be a quadruple bar `||||`"
            ),
        }
    }
}

impl std::error::Error for Error {}
