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
use flashcard::{Flashcard, FlashcardTypes};

use crate::flashcard::{FrontBack, List, Reveal};

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
    let mut card_type: Option<FlashcardTypes> = None;
    let mut indices: Vec<usize> = vec![];
    let len = document.len();
    for (i, c) in document.chars().enumerate() {
        use flashcard::FlashcardTypes as FT;
        match (last_char, c) {
            ('{', '{') => {
                if !building {
                    indices.push(i); // Don't include the `{`
                    building = true;
                }
            }
            ('}', '}') => {
                if building {
                    // Construct the card
                    if let Some(ref some_card_type) = card_type {
                        use flashcard::Flashcard as FC;
                        let card: Flashcard = match some_card_type {
                            FT::FrontBack => FC::FrontBack(FrontBack::new(
                                document[indices[0] + 1..indices[1] - 1].to_string(),
                                document[indices[1]..i - 1].to_string(),
                            )),
                            FT::Reveal => FC::Reveal(Reveal::new(
                                document[indices[0] + 1..indices[1] - 1].to_string(),
                                document[indices[1] + 1..indices[2] - 1].to_string(),
                                document[indices[2] + 1..i - 1].to_string(),
                            )),
                            FT::OrderedList => {
                                indices.push(i);
                                let entries = indices
                                    .iter()
                                    .skip(2)
                                    .zip(indices.iter().skip(3))
                                    .map(|(&a, &b)| document[a + 1..b - 1].to_string())
                                    .collect();

                                FC::OrderedList(List::new(
                                    document[indices[0] + 1..indices[1] - 1].to_string(),
                                    entries,
                                ))
                            }
                        };

                        cards.push(card);
                        indices = vec![];
                        card_type = None;
                    } else {
                        return Err(error::Error::EmptyCard);
                    }
                    building = false;
                } else {
                    return Err(error::Error::NoOpeningBrackets);
                }
            }
            ('|', '>') => {
                if building {
                    card_type = Some(FT::OrderedList);
                    indices.push(i);
                }
            }
            ('|', '|') => {
                if building {
                    indices.push(i);
                    if card_type.is_none() {
                        card_type = Some(FT::Reveal);
                    }
                }
            }
            ('|', _) => {
                if building && card_type.is_none() {
                    card_type = Some(FT::FrontBack);
                    indices.push(i); // Ignore the `|`
                }
            }
            ('1'..='9', '.') => {
                if building {
                    if let Some(ref card_type) = card_type {
                        if let FT::OrderedList = card_type {
                            indices.push(i);
                        }
                    }
                }
            }
            _ => (),
        }
        last_char = c;
        if i == len - 1 && building == true {
            return Err(error::Error::UnclosedBrackets);
        }
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
