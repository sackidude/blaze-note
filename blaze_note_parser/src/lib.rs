#![forbid(unsafe_code)]
//! # blaze-note
//!
//! This is a suite of services for note taking and semi-automatic flashcard
//! genertion using markdown and some custom syntax.
//!
//! ## Example
//!
//! ```
//! use blaze_note_parser::{parse_flashcards, compile_to_html};
//! use std::error::Error;
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let document = String::from("# Hello, _World_!");
//!     let flashcards = parse_flashcards(&document)?;
//!     let html = compile_to_html(document)?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! Check out `testing` directory for more examples

pub mod error;
pub mod flashcard;

use error::Result;
use flashcard::{Flashcard, FlashcardBuilder};

/// parses and returns all of the flashcards in a document.
///
/// This accepts a string-slice which means it should be run before and eventual
///  `compile_to_html` or `compile_to_markdown`.
///
/// ## Example
///
/// ```
/// use blaze_note_parser::parse_flashcards;
/// use std::error::Error;
/// fn main() -> Result<(), Box<dyn Error>> {
///     let document = String::from("# Hello, _World_!");
///     let flashcards = parse_flashcards(&document)?;
///
///     
///     Ok(())
/// }
/// ```
///
/// ## Errors
///
/// This returns an error when the document isn't conforming to the syntax of
/// blaze-note, the function fails in the following scenarios:
///
/// 1. Opened but never closed double brackets `{{`
///
pub fn parse_flashcards(document: &str) -> Result<Vec<Flashcard>> {
    let mut cards: Vec<Flashcard> = vec![];

    // I don't love this solution, would like to have a sliding windows and one
    // very nice match statement but w.e. this works.
    let mut last_char = '\0';
    let mut building = false;
    let mut builder = &mut FlashcardBuilder::new();
    for (i, c) in document.chars().enumerate() {
        use flashcard::FlashcardTypes as FT;
        match (last_char, c) {
            ('{', '{') => {
                if !building {
                    builder = builder.push_index(i);
                    building = true;
                }
            }
            ('}', '}') => {
                if building {
                    let card = builder.build()?;
                    cards.push(card);
                } else {
                    // CONSIDER!: Could return error here
                }
            }
            ('|', '>') => {
                if building {
                    builder.card_type(FT::OrderedList);
                }
            }
            ('|', '|') => {
                if building {
                    builder.card_type(FT::Reveal);
                }
            }
            ('|', _) => {
                if building {
                    builder.card_type(FT::FrontBack);
                }
            }
            _ => (),
        }
        last_char = c;
    }

    Ok(cards)
}

pub fn compile_to_html(document: String) -> Result<String> {
    let compiled_markdown = compile_to_markdown(document)?;

    Ok(markdown::to_html(&compiled_markdown))
}

pub fn compile_to_markdown(document: String) -> Result<String> {
    Ok("poop".into())
}
