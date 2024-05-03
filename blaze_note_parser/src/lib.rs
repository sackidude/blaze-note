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
//!     let html = compile_to_html(&document)?;
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
use markdown::{CompileOptions, Options};

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
                            FT::Reveal => {
                                if indices[2] - 2 < indices[1] {
                                    // This represents a |||| case for this,
                                    // which is considered malformed input.
                                    return Err(error::Error::MalformedBars);
                                }
                                FC::Reveal(Reveal::new(
                                    document[indices[0] + 1..indices[1] - 1].to_string(),
                                    document[indices[1] + 1..indices[2] - 1].to_string(),
                                    document[indices[2] + 1..i - 1].to_string(),
                                ))
                            }
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

pub fn compile_to_html(document: &str) -> Result<String> {
    let compiled_markdown = compile_to_markdown(document)?;

    Ok(markdown::to_html_with_options(
        &compiled_markdown,
        &Options {
            compile: CompileOptions {
                allow_dangerous_html: true,
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap())
}

pub fn compile_to_markdown(document: &str) -> Result<String> {
    if document.len() == 0 {
        return Ok(String::new());
    }

    // This algorithm should be of O(n) time complexity and O(n) memory complexity
    let mut in_card = false;
    let mut previous = '\0';
    let mut card_type = FlashcardTypes::FrontBack;
    let mut buffer: Vec<String> = vec![];
    let mut compiled = String::new();
    for current in document.chars() {
        match (previous, current, in_card) {
            ('{', '{', false) => {
                compiled.pop();
                in_card = true;
                buffer.push(String::new());
            }
            ('}', '}', true) => {
                let len = buffer.len();
                buffer[len - 1].pop();
                in_card = false;
                compiled.push_str(&create_md_card(&card_type, &buffer)?);

                // Reset for next card
                buffer = vec![];
                card_type = FlashcardTypes::FrontBack;
            }
            ('|', '|', true) => {
                card_type = FlashcardTypes::Reveal;
            }
            ('|', '>', true) => {
                card_type = FlashcardTypes::OrderedList;
            }
            (_, '|', true) => buffer.push(String::new()),
            (_, _, false) => {
                compiled.push(current);
            }
            (_, _, true) => {
                let len = buffer.len();
                buffer[len - 1].push(current)
            }
        }
        previous = current;
    }

    if in_card {
        Err(crate::error::Error::UnclosedBrackets)?
    }

    Ok(compiled)
}

fn create_md_card(card_type: &FlashcardTypes, buffer: &Vec<String>) -> Result<String> {
    let len = buffer.len();
    Ok(match card_type {
        FlashcardTypes::FrontBack => {
            if len == 2 {
                format!("<span class=\"card front-back-card\"><span class=\"front\">{}</span><span class=\"back\">{}</span></span>", buffer[0], buffer[1])
            } else {
                Err(crate::error::Error::MalformedBars)?
            }
        }
        FlashcardTypes::Reveal => {
            if len == 3 {
                format!(
                    "<span class=\"card reveal-card\">{}<span class=\"hidden\">{}</span>{}</span>",
                    buffer[0], buffer[1], buffer[2]
                )
            } else {
                Err(crate::error::Error::MalformedBars)?
            }
        }
        FlashcardTypes::OrderedList => {
            if len == 2 {
                format!("<span class=\"card list-card\"><span class=\"question\">{}</span><span class=\"entries\">{}</span></span>", buffer[0], buffer[1])
            } else {
                Err(crate::error::Error::MalformedBars)?
            }
        }
    })
}
