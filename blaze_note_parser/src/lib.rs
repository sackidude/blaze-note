#![forbid(unsafe_code)]
//! # blaze-note
//!
//! This is a suite of services for note taking and semi-automatic flashcard
//! genertion using markdown and some custom syntax.
//!

pub mod error;
pub mod flashcard;

use error::Result;
use flashcard::Flashcard;

/// Parse a document into a `Note`
///
/// The base of the whole library used in other helper functions.
///
/// ## Errors
///
/// Can error when the _note_ syntax is invalid. Can't error due to markdown
/// because it can't error.
///
/// ## Examples
///
/// ```
/// use blaze_note_parser::parse;
/// use std::error::Error;
///
/// fn main() -> Result<(), Box<dyn Error>> {
///     let note = parse("# Hello, _World_!")?;
///     Ok(())
/// }
/// ```
pub fn parse(document: &str) -> Result<Note> {
    let html = markdown::to_html(document);
    let cards = vec![];
    let note = Note { html, cards };

    Ok(note)
}

pub struct Note {
    html: String,
    cards: Vec<Flashcard>,
}

impl Note {
    pub fn get_html(&self) -> &str {
        &self.html
    }

    pub fn get_cards(&self) -> &[Flashcard] {
        &self.cards
    }
}
